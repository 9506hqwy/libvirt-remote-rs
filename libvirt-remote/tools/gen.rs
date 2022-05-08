use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use syn;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("Not specify file path")?;
    let contents = fs::read_to_string(&path)?;

    let source = TokenStream::from_str(&contents)?;
    let client = gen(source)?;

    println!("{}", client);
    Ok(())
}

fn gen(stream: TokenStream) -> Result<String, Box<dyn Error>> {
    let file: syn::File = syn::parse2(stream)?;

    let mut models = vec![];
    let mut procedures = None;
    for item in &file.items {
        if let syn::Item::Struct(model) = item {
            models.push(format!("{}", model.ident));
        }

        if let syn::Item::Enum(e) = item {
            if e.ident == "RemoteProcedure" {
                procedures = Some(e);
            }
        }
    }

    let procedures = procedures.ok_or("Not found `RemoteProcedure`.")?;

    let mut procs = vec![];
    for procedure in &procedures.variants {
        let method_str = format!("{}", procedure.ident);
        if let Some(method) = method_str.strip_prefix("RemoteProc") {
            let method_ret = models
                .iter()
                .find(|&m| m == &format!("Remote{}Ret", method));
            let method_args = models
                .iter()
                .find(|&m| m == &format!("Remote{}Args", method));
            procs.push((method.to_string(), method_args, method_ret));
        }
    }

    let mut calls = vec![];
    for proc in procs {
        let method_name = format_ident!("{}", snake_case(&proc.0));
        let flag = format_ident!("RemoteProc{}", proc.0);

        let fn_stmt = if let Some(arg) = proc.1 {
            let args_type = format_ident!("{}", arg);
            quote! {
                #method_name(&mut self, args: binding::#args_type)
            }
        } else {
            quote! {
                #method_name(&mut self)
            }
        };

        let req_stmt = if let Some(arg) = proc.1 {
            let args_type = format_ident!("{}", arg);
            quote! {
                let req: Option<binding::#args_type> = Some(args);
            }
        } else {
            quote! {
                let req: Option<()> = None;
            }
        };

        let ret_stmt = if let Some(ret) = proc.2 {
            let ret_type = format_ident!("{}", ret);
            quote! { binding::#ret_type }
        } else {
            quote! { () }
        };

        let proc_stmt = if let Some(_) = proc.2 {
            quote! {
                let res: Option<#ret_stmt> = call(self, binding::RemoteProcedure::#flag, req)?;
                Ok(res.unwrap())
            }
        } else {
            quote! {
                let _res: Option<()> = call(self, binding::RemoteProcedure::#flag, req)?;
                Ok(())
            }
        };

        calls.push(quote! {
            fn #fn_stmt -> Result<#ret_stmt, Error> {
                trace!("{}", stringify!(#method_name));
                #req_stmt
                #proc_stmt
            }
        });
    }

    let mut msgs = vec![];
    for model in models {
        if !model.ends_with("Msg") {
            continue;
        }

        let method_name = format_ident!("{}", snake_case(&model.strip_prefix("Remote").unwrap()));

        let fn_stmt = quote! {
            #method_name(&mut self)
        };

        let ret_type = format_ident!("{}", model);
        let ret_stmt = quote! { binding::#ret_type };

        let proc_stmt = quote! {
            let res: Option<#ret_stmt> = msg(self)?;
            Ok(res.unwrap())
        };

        msgs.push(quote! {
            fn #fn_stmt -> Result<#ret_stmt, Error> {
                trace!("{}", stringify!(#method_name));
                #proc_stmt
            }
        });
    }

    let client = quote! {
        use crate::binding;
        use crate::error::Error;
        use crate::protocol;
        use log::trace;
        use serde::{de::DeserializeOwned, Serialize};
        use std::io::{Read, Write};
        use std::net::TcpStream;
        #[cfg(target_family = "unix")]
        use std::os::unix::net::UnixStream;

        pub trait ReadWrite: Read + Write {}
        impl ReadWrite for TcpStream {}
        #[cfg(target_family = "unix")]
        impl ReadWrite for UnixStream {}

        pub struct Client {
            inner: Box<dyn ReadWrite>,
            serial: u32,
        }

        impl Client {
            pub fn new(socket: impl ReadWrite + 'static) -> Self {
                Client {
                    inner: Box::new(socket),
                    serial: 0,
                }
            }
        }

        impl Libvirt for Client {
            fn inner(&mut self) -> &mut Box<dyn ReadWrite> {
                &mut self.inner
            }

            fn serial(&self) -> u32 {
                self.serial
            }

            fn serial_add(&mut self, value: u32) {
                self.serial += value;
            }
        }

        pub trait Libvirt {
            fn inner(&mut self) -> &mut Box<dyn ReadWrite>;

            fn serial(&self) -> u32;

            fn serial_add(&mut self, value: u32);

            #(#calls)*

            #(#msgs)*
        }

        fn call<S, D>(
            client: &mut (impl Libvirt + ?Sized),
            procedure: binding::RemoteProcedure,
            args: Option<S>,
        ) -> Result<Option<D>, Error>
        where
            S: Serialize,
            D: DeserializeOwned,
        {
            client.serial_add(1);

            let mut req_len: u32 = 4;

            let req_header = protocol::VirNetMessageHeader {
                prog: binding::REMOTE_PROGRAM,
                vers: binding::REMOTE_PROTOCOL_VERSION,
                proc: procedure as i32,
                r#type: protocol::VirNetMessageType::VirNetCall,
                serial: client.serial(),
                status: protocol::VirNetMessageStatus::VirNetOk,
            };
            let req_header_bytes = serde_xdr::to_bytes(&req_header).map_err(Error::SerializeError)?;
            req_len += req_header_bytes.len() as u32;

            let mut args_bytes = None;
            if let Some(args) = &args {
                let body = serde_xdr::to_bytes(args).map_err(Error::SerializeError)?;
                req_len += body.len() as u32;
                args_bytes = Some(body);
            }

            client
                .inner()
                .write_all(&req_len.to_be_bytes())
                .map_err(Error::SendError)?;
            client
                .inner()
                .write_all(&req_header_bytes)
                .map_err(Error::SendError)?;
            if let Some(args_bytes) = &args_bytes {
                client
                    .inner()
                    .write_all(args_bytes)
                    .map_err(Error::SendError)?;
            }

            let mut res_len_bytes = [0; 4];
            client
                .inner()
                .read_exact(&mut res_len_bytes)
                .map_err(Error::ReceiveError)?;
            let res_len = u32::from_be_bytes(res_len_bytes) as usize;

            let mut res_header_bytes = [0; 24];
            client
                .inner()
                .read_exact(&mut res_header_bytes)
                .map_err(Error::ReceiveError)?;
            let res_header = serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
                .map_err(Error::DeserializeError)?;

            if res_len == (4 + res_header_bytes.len()) {
                return Ok(None);
            }

            let mut res_body_bytes = vec![0u8; res_len - 4 - res_header_bytes.len()];
            client
                .inner()
                .read_exact(&mut res_body_bytes)
                .map_err(Error::ReceiveError)?;
            if res_header.status == protocol::VirNetMessageStatus::VirNetError {
                let res = serde_xdr::from_bytes::<protocol::VirNetMessageError>(&res_body_bytes)
                    .map_err(Error::DeserializeError)?;
                Err(Error::ProtocolError(res))
            } else {
                let res = serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
                Ok(Some(res))
            }
        }

        fn msg<D>(
            client: &mut (impl Libvirt + ?Sized),
        ) -> Result<Option<D>, Error>
        where
            D: DeserializeOwned,
        {
            let mut res_len_bytes = [0; 4];
            client
                .inner()
                .read_exact(&mut res_len_bytes)
                .map_err(Error::ReceiveError)?;
            let res_len = u32::from_be_bytes(res_len_bytes) as usize;

            let mut res_header_bytes = [0; 24];
            client
                .inner()
                .read_exact(&mut res_header_bytes)
                .map_err(Error::ReceiveError)?;
            let res_header = serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
                .map_err(Error::DeserializeError)?;

            if res_len == (4 + res_header_bytes.len()) {
                return Ok(None);
            }

            let mut res_body_bytes = vec![0u8; res_len - 4 - res_header_bytes.len()];
            client
                .inner()
                .read_exact(&mut res_body_bytes)
                .map_err(Error::ReceiveError)?;
            if res_header.status == protocol::VirNetMessageStatus::VirNetError {
                let res = serde_xdr::from_bytes::<protocol::VirNetMessageError>(&res_body_bytes)
                    .map_err(Error::DeserializeError)?;
                Err(Error::ProtocolError(res))
            } else {
                let res = serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
                Ok(Some(res))
            }
        }
    };

    Ok(client.to_string())
}

fn capitalize(value: &str) -> String {
    value
        .chars()
        .enumerate()
        .map(|(index, c)| match index {
            0 => c.to_ascii_uppercase(),
            _ => c.to_ascii_lowercase(),
        })
        .collect()
}

fn snake_case(value: &str) -> String {
    let capitalized = value
        .split_inclusive(|c| char::is_lowercase(c) || char::is_numeric(c))
        .map(|c| {
            match c.len() {
                1 => c.to_string(),
                // 複数の大文字が連続する場合は先頭のみ大文字にする。
                // 以下のように変換させる。
                // ex)
                //    userID ->  userId -> user_id
                _ => capitalize(c),
            }
        })
        .fold("".to_string(), |mut acc, x| {
            acc.push_str(&x);
            acc
        });

    let mut v = "".to_string();

    let mut index = 0;
    for (next, _) in capitalized.match_indices(char::is_uppercase) {
        match index {
            0 => v.push_str(&capitalized[index..next].to_ascii_lowercase()),
            _ => v.push_str(&with_underscore(&capitalized[index..next])),
        }

        index = next;
    }

    match index {
        0 => v.push_str(&capitalized.to_ascii_lowercase()),
        _ => v.push_str(&with_underscore(&capitalized[index..])),
    }

    v
}

fn with_underscore(value: &str) -> String {
    let mut v = "_".to_string();
    v.push_str(&value.to_ascii_lowercase());
    v
}
