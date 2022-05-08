use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use syn;

const UN_DECONSTRUCTING: [&str; 7] = [
    "RemoteDomainEventBlockThresholdMsg",
    "RemoteDomainEventDiskChangeMsg",
    "RemoteDomainEventGraphicsMsg",
    "RemoteDomainGetJobInfoRet",
    "RemoteDomainInterfaceStatsRet",
    "RemoteDomainMigratePerform3Args",
    "RemoteNodeGetInfoRet",
];

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("Not specify file path")?;
    let contents = fs::read_to_string(&path)?;

    let source = TokenStream::from_str(&contents)?;
    let client = gen(source, false)?;

    println!("{}", client);
    Ok(())
}

fn gen(stream: TokenStream, wrapped: bool) -> Result<String, Box<dyn Error>> {
    let (procedures, models) = parse_file(stream)?;

    let mut calls = vec![];
    for (name, args, ret) in parse_call_method(&procedures, &models) {
        let method_name = format_ident!("{}", snake_case(&name));
        let flag = format_ident!("RemoteProc{}", &name);

        let fn_args = gen_fn_args(&method_name, args.as_deref(), wrapped, &models);
        let res_type = gen_res_type(ret.as_deref(), wrapped, &models);
        let req_stmt = gen_req_stmt(args.as_deref(), wrapped, &models);
        let proc_stmt = gen_proc_stmt(
            quote! { call(self, RemoteProcedure::#flag, req)? },
            ret.as_deref(),
            wrapped,
            &models,
        );

        calls.push(quote! {
            fn #fn_args -> Result<#res_type, Error> {
                trace!("{}", stringify!(#method_name));
                #req_stmt
                #proc_stmt
            }
        });
    }

    let mut msgs = vec![];
    for (name, ret) in parse_msg_method(&models) {
        let method_name = format_ident!("{}", snake_case(&name));

        let fn_args = gen_fn_args(&method_name, None, wrapped, &models);
        let res_type = gen_res_type(Some(&ret), wrapped, &models);
        let proc_stmt = gen_proc_stmt(quote! { msg(self)? }, Some(&ret), wrapped, &models);

        msgs.push(quote! {
            fn #fn_args -> Result<#res_type, Error> {
                trace!("{}", stringify!(#method_name));
                #proc_stmt
            }
        });
    }

    let client = quote! {
        use crate::binding::*;
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

        pub enum VirNetRequest<S>
        where
            S: Serialize,
        {
            Data(S),
            Stream(VirNetStream),
        }

        pub enum VirNetResponse<D>
        where
            D: DeserializeOwned,
        {
            Data(D),
            Stream(VirNetStream),
        }

        pub enum VirNetStream {
            Hole(protocol::VirNetStreamHole),
            Raw(Vec<u8>),
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

            fn download(&mut self) -> Result<Option<VirNetStream>, Error> {
                download(self)
            }

            #(#calls)*

            #(#msgs)*

            fn storage_vol_upload_data(&mut self, buf: &[u8]) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_data));
                upload(self, RemoteProcedure::RemoteProcStorageVolUpload, buf)
            }

            fn storage_vol_upload_hole(&mut self, length: i64, flags: u32) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_hole));
                send_hole(
                    self,
                    RemoteProcedure::RemoteProcStorageVolUpload,
                    length,
                    flags,
                )
            }

            fn storage_vol_upload_complete(&mut self) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_complete));
                upload_completed(
                    self,
                    RemoteProcedure::RemoteProcStorageVolUpload,
                )
            }
        }

        fn call<S, D>(
            client: &mut (impl Libvirt + ?Sized),
            procedure: RemoteProcedure,
            args: Option<S>,
        ) -> Result<Option<D>, Error>
        where
            S: Serialize,
            D: DeserializeOwned,
        {
            client.serial_add(1);
            send(
                client,
                procedure,
                protocol::VirNetMessageType::VirNetCall,
                protocol::VirNetMessageStatus::VirNetOk,
                args.map(|a| VirNetRequest::Data(a)),
            )?;
            match recv(client)? {
                Some(VirNetResponse::Data(res)) => Ok(Some(res)),
                None => Ok(None),
                _ => unreachable!(),
            }
        }

        fn msg<D>(
            client: &mut (impl Libvirt + ?Sized),
        ) -> Result<Option<D>, Error>
        where
            D: DeserializeOwned,
        {
            match recv(client)? {
                Some(VirNetResponse::Data(res)) => Ok(Some(res)),
                None => Ok(None),
                _ => unreachable!(),
            }
        }

        fn download(client: &mut (impl Libvirt + ?Sized)) -> Result<Option<VirNetStream>, Error> {
            let body: Option<VirNetResponse<()>> = recv(client)?;

            match body {
                Some(VirNetResponse::Stream(stream)) => Ok(Some(stream)),
                None => Ok(None),
                _ => unreachable!(),
            }
        }

        fn upload(
            client: &mut (impl Libvirt + ?Sized),
            procedure: RemoteProcedure,
            buf: &[u8],
        ) -> Result<(), Error> {
            let bytes = VirNetStream::Raw(buf.to_vec());
            let req: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(bytes));
            send(
                client,
                procedure,
                protocol::VirNetMessageType::VirNetStream,
                protocol::VirNetMessageStatus::VirNetContinue,
                req,
            )?;
            Ok(())
        }

        fn send_hole(
            client: &mut (impl Libvirt + ?Sized),
            procedure: RemoteProcedure,
            length: i64,
            flags: u32,
        ) -> Result<(), Error> {
            let hole = VirNetStream::Hole(protocol::VirNetStreamHole { length, flags });
            let args: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(hole));
            send(
                client,
                procedure,
                protocol::VirNetMessageType::VirNetStreamHole,
                protocol::VirNetMessageStatus::VirNetContinue,
                args,
            )?;
            Ok(())
        }

        fn upload_completed(
            client: &mut (impl Libvirt + ?Sized),
            procedure: RemoteProcedure,
        ) -> Result<(), Error> {
            let req: Option<VirNetRequest<()>> = None;
            send(
                client,
                procedure,
                protocol::VirNetMessageType::VirNetStream,
                protocol::VirNetMessageStatus::VirNetOk,
                req,
            )?;
            let _res: Option<VirNetResponse<()>> = recv(client)?;
            Ok(())
        }

        fn send<S>(
            client: &mut (impl Libvirt + ?Sized),
            procedure: RemoteProcedure,
            req_type: protocol::VirNetMessageType,
            req_status: protocol::VirNetMessageStatus,
            args: Option<VirNetRequest<S>>,
        ) -> Result<usize, Error>
        where
            S: Serialize,
        {
            let mut req_len: u32 = 4;

            let req_header = protocol::VirNetMessageHeader {
                prog: REMOTE_PROGRAM,
                vers: REMOTE_PROTOCOL_VERSION,
                proc: procedure as i32,
                r#type: req_type,
                serial: client.serial(),
                status: req_status,
            };
            let req_header_bytes = serde_xdr::to_bytes(&req_header).map_err(Error::SerializeError)?;
            req_len += req_header_bytes.len() as u32;

            let mut args_bytes = None;
            match args {
                Some(VirNetRequest::Data(data)) => {
                    let body = serde_xdr::to_bytes(&data).map_err(Error::SerializeError)?;
                    req_len += body.len() as u32;
                    args_bytes = Some(body);
                },
                Some(VirNetRequest::Stream(VirNetStream::Raw(bytes))) => {
                    req_len += bytes.len() as u32;
                    args_bytes = Some(bytes);
                },
                Some(VirNetRequest::Stream(VirNetStream::Hole(hole))) => {
                    let body = serde_xdr::to_bytes(&hole).map_err(Error::SerializeError)?;
                    req_len += body.len() as u32;
                    args_bytes = Some(body);
                },
                None => { },
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

            Ok(req_len as usize)
        }

        fn recv<D>(
            client: &mut (impl Libvirt + ?Sized),
        ) -> Result<Option<VirNetResponse<D>>, Error>
        where
            D: DeserializeOwned,
        {
            let res_len = read_pkt_len(client)?;
            let res_header = read_res_header(client)?;

            let body_len = res_len - 28;
            if body_len == 0 {
                return Ok(None);
            }

            Ok(Some(read_res_body(client, &res_header, body_len)?))
        }

        fn read_pkt_len(client: &mut (impl Libvirt + ?Sized)) -> Result<usize, Error> {
            let mut res_len_bytes = [0; 4];
            client
                .inner()
                .read_exact(&mut res_len_bytes)
                .map_err(Error::ReceiveError)?;
            Ok(u32::from_be_bytes(res_len_bytes) as usize)
        }

        fn read_res_header(client: &mut (impl Libvirt + ?Sized)) -> Result<protocol::VirNetMessageHeader, Error> {
            let mut res_header_bytes = [0; 24];
            client
                .inner()
                .read_exact(&mut res_header_bytes)
                .map_err(Error::ReceiveError)?;
            serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
                .map_err(Error::DeserializeError)
        }

        fn read_res_body<D>(
            client: &mut (impl Libvirt + ?Sized),
            res_header: &protocol::VirNetMessageHeader,
            size: usize,
        ) -> Result<VirNetResponse<D>, Error>
        where
            D: DeserializeOwned,
        {
            let mut res_body_bytes = vec![0u8; size];
            client
                .inner()
                .read_exact(&mut res_body_bytes)
                .map_err(Error::ReceiveError)?;
            if res_header.status == protocol::VirNetMessageStatus::VirNetError {
                let res = serde_xdr::from_bytes::<protocol::VirNetMessageError>(&res_body_bytes)
                    .map_err(Error::DeserializeError)?;
                Err(Error::ProtocolError(res))
            } else {
                match res_header.r#type {
                    protocol::VirNetMessageType::VirNetReply | protocol::VirNetMessageType::VirNetMessage => {
                        let data = serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
                        Ok(VirNetResponse::Data(data))
                    }
                    protocol::VirNetMessageType::VirNetStream => {
                        let stream = VirNetStream::Raw(res_body_bytes);
                        Ok(VirNetResponse::Stream(stream))
                    }
                    protocol::VirNetMessageType::VirNetStreamHole => {
                        let hole = serde_xdr::from_bytes::<protocol::VirNetStreamHole>(&res_body_bytes)
                            .map_err(Error::DeserializeError)?;
                        let stream = VirNetStream::Hole(hole);
                        Ok(VirNetResponse::Stream(stream))
                    }
                    _ => unreachable!(),
                }
            }
        }
    };

    Ok(client.to_string())
}

fn parse_file(
    stream: TokenStream,
) -> Result<(syn::ItemEnum, HashMap<String, syn::ItemStruct>), Box<dyn Error>> {
    let file: syn::File = syn::parse2(stream)?;

    let mut models = HashMap::new();
    let mut procedures = None;
    for item in &file.items {
        if let syn::Item::Struct(model) = item {
            models.insert(format!("{}", model.ident), model.clone());
        }

        if let syn::Item::Enum(e) = item {
            if e.ident == "RemoteProcedure" {
                procedures = Some(e);
            }
        }
    }

    let procedures = procedures.ok_or("Not found `RemoteProcedure`.")?.clone();

    Ok((procedures, models))
}

fn parse_call_method(
    procedures: &syn::ItemEnum,
    models: &HashMap<String, syn::ItemStruct>,
) -> Vec<(String, Option<String>, Option<String>)> {
    let mut procs = vec![];
    for procedure in &procedures.variants {
        let method_str = format!("{}", procedure.ident);
        if let Some(method) = method_str.strip_prefix("RemoteProc") {
            let method_args = models
                .keys()
                .find(|&m| m == &format!("Remote{}Args", method))
                .cloned();
            let method_ret = models
                .keys()
                .find(|&m| m == &format!("Remote{}Ret", method))
                .cloned();
            procs.push((method.to_string(), method_args, method_ret));
        }
    }
    procs
}

fn parse_msg_method(models: &HashMap<String, syn::ItemStruct>) -> Vec<(String, String)> {
    let mut procs = vec![];
    for model in models.keys() {
        if !model.ends_with("Msg") {
            continue;
        }

        procs.push((
            model.strip_prefix("Remote").unwrap().to_string(),
            model.to_string(),
        ));
    }
    procs.sort_by_key(|m| m.1.clone());
    procs
}

fn gen_fn_args(
    name: &Ident,
    model: Option<&str>,
    wrapped: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if let Some(model) = model {
        if wrapped || undeconstructing(model) {
            let model_ident = format_ident!("{}", model);
            quote! {
                #name(&mut self, args: #model_ident)
            }
        } else {
            let model = models.get(model).unwrap();
            let params = syn_fields_to_sig_params(model);
            quote! {
                #name(&mut self, #(#params),* )
            }
        }
    } else {
        quote! {
            #name(&mut self)
        }
    }
}

fn gen_res_type(
    model: Option<&str>,
    wrapped: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if let Some(model) = model {
        if wrapped || undeconstructing(model) {
            let model_ident = format_ident!("{}", model);
            quote! { #model_ident }
        } else {
            let model = models.get(model).unwrap();
            let types = syn_fields_to_sig_types(model);
            if types.len() > 1 {
                quote! { (#(#types),*) }
            } else {
                quote! { #(#types),* }
            }
        }
    } else {
        quote! { () }
    }
}

fn gen_req_stmt(
    model: Option<&str>,
    wrapped: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if let Some(model) = model {
        let model_ident = format_ident!("{}", model);

        if wrapped || undeconstructing(model) {
            quote! {
                let req: Option<#model_ident> = Some(args);
            }
        } else {
            let model = models.get(model).unwrap();
            let fields = syn_fields_to_sig_fields(model);
            quote! {
                let req: Option<#model_ident> = Some(#model_ident {
                    #(#fields),*
                });
            }
        }
    } else {
        quote! {
            let req: Option<()> = None;
        }
    }
}

fn gen_proc_stmt(
    proc: TokenStream,
    model: Option<&str>,
    wrapped: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if let Some(model) = model {
        let model_ident = format_ident!("{}", model);

        if wrapped || undeconstructing(model) {
            quote! {
                let res: Option<#model_ident> = #proc;
                Ok(res.unwrap())
            }
        } else {
            let model = models.get(model).unwrap();
            let fields = syn_fields_to_sig_fields(model);

            let call_stmt = quote! {
                let res: Option<#model_ident> = #proc;
                let res = res.unwrap();
                let #model_ident { #(#fields),* } = res;
            };

            if fields.len() > 1 {
                quote! {
                    #call_stmt
                    Ok((#(#fields),*))
                }
            } else {
                quote! {
                    #call_stmt
                    Ok(#(#fields),*)
                }
            }
        }
    } else {
        quote! {
            let _res: Option<()> = #proc;
            Ok(())
        }
    }
}

fn syn_fields_to_sig_fields(model: &syn::ItemStruct) -> Vec<TokenStream> {
    let mut args = vec![];
    if let syn::Fields::Named(fields) = &model.fields {
        for field in fields.named.iter() {
            let ident = field.ident.as_ref().unwrap();
            args.push(quote! { #ident });
        }
    }
    args
}

fn syn_fields_to_sig_types(model: &syn::ItemStruct) -> Vec<TokenStream> {
    let mut args = vec![];
    if let syn::Fields::Named(fields) = &model.fields {
        for field in fields.named.iter() {
            let ty = &field.ty;
            args.push(quote! { #ty });
        }
    }
    args
}

fn syn_fields_to_sig_params(model: &syn::ItemStruct) -> Vec<TokenStream> {
    let mut args = vec![];
    if let syn::Fields::Named(fields) = &model.fields {
        for field in fields.named.iter() {
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            args.push(quote! { #ident: #ty });
        }
    }
    args
}

fn undeconstructing(model: &str) -> bool {
    UN_DECONSTRUCTING.iter().any(|&m| m == model)
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
