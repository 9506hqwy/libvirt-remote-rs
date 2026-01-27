use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const STREAM_PROCS: [&str; 5] = [
    "DomainMigratePrepareTunnel",
    "DomainOpenConsole",
    "StorageVolUpload",
    "StorageVolDownload",
    // TODO:
    // "DomainScreenshot",
    // "DomainMigratePrepareTunnel3",
    "DomainOpenChannel",
];

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
    let contents = fs::read_to_string(path)?;

    let source = TokenStream::from_str(&contents)?;
    let client = gen_code(source, false)?;

    println!("{client}");
    Ok(())
}

fn gen_code(stream: TokenStream, wrapped: bool) -> Result<String, Box<dyn Error>> {
    let (procedures, models) = parse_file(stream)?;

    let mut calls = vec![];
    for (name, args, ret) in parse_call_method(&procedures, &models) {
        let stream = stream_procs(&name);
        let method_name = format_ident!("{}", snake_case(&name));
        let flag = format_ident!("RemoteProc{}", &name);

        let xdr_req_type = match args.as_deref() {
            Some(model) => {
                let model_ident = format_ident!("{}", model);
                quote! { #model_ident }
            }
            _ => quote! { () },
        };

        let xdr_res_type = match ret.as_deref() {
            Some(model) => {
                let model_ident = format_ident!("{}", model);
                quote! { #model_ident }
            }
            _ => quote! { () },
        };

        let stream_arg = if stream {
            quote! { true }
        } else {
            quote! { false }
        };

        let call_proc = quote! { call::<#xdr_req_type, #xdr_res_type>(self, RemoteProcedure::#flag, #stream_arg, req)? };

        let fn_args = gen_fn_args(&method_name, args.as_deref(), wrapped, &models);
        let res_type = gen_res_type(ret.as_deref(), wrapped, stream, &models);
        let req_stmt = gen_req_stmt(args.as_deref(), wrapped, &models);
        let proc_stmt = gen_proc_stmt(call_proc, ret.as_deref(), wrapped, stream, &models);

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
        let stream = stream_procs(&name);
        let method_name = format_ident!("{}", snake_case(&name));

        let fn_args = gen_fn_args(&method_name, None, wrapped, &models);
        let res_type = gen_res_type(Some(&ret), wrapped, stream, &models);
        let proc_stmt = gen_proc_stmt(quote! { msg(self)? }, Some(&ret), wrapped, stream, &models);

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
        use serde::{Serialize, de::DeserializeOwned};
        use std::collections::HashMap;
        use std::io::ErrorKind;
        use std::io::{Read, Write};
        use std::net::TcpStream;
        #[cfg(target_family = "unix")]
        use std::os::unix::net::UnixStream;
        use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
        use std::sync::mpsc::{Receiver, Sender, channel};
        use std::sync::{Arc, Mutex};
        use std::thread::{self, JoinHandle};
        use std::time::Duration;

        pub trait ReadWrite: Read + Write + Send {
            fn clone(&self) -> Result<Box<dyn ReadWrite>, Error>;
        }
        impl ReadWrite for TcpStream {
            fn clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
                let s = self.try_clone().map_err(Error::SocketError)?;
                Ok(Box::new(s))
            }
        }
        #[cfg(target_family = "unix")]
        impl ReadWrite for UnixStream {
            fn clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
                let s = self.try_clone().map_err(Error::SocketError)?;
                Ok(Box::new(s))
            }
        }

        pub struct Client {
            inner: Box<dyn ReadWrite>,
            serial: Arc<AtomicU32>,
            receiver: Arc<JoinHandle<()>>,
            receiver_run: Arc<AtomicBool>,
            channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
        }

        pub struct VirNetStreamResponse {
            inner: Box<dyn ReadWrite>,
            channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
            receiver: Receiver<VirNetResponseRaw>,
            header: protocol::VirNetMessageHeader,
        }

        pub enum VirNetRequest<S>
        where
            S: Serialize,
        {
            Data(S),
            Stream(VirNetStream),
        }

        pub struct VirNetResponseRaw {
            header: protocol::VirNetMessageHeader,
            body: Option<Vec<u8>>,
        }

        pub struct VirNetResponseSet<D> {
            receiver: Option<Receiver<VirNetResponseRaw>>,
            header: protocol::VirNetMessageHeader,
            body: Option<D>,
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
                let receiver_run = Arc::new(AtomicBool::new(true));
                let channels = Arc::new(Mutex::new(HashMap::new()));

                let t_receiver_run = Arc::clone(&receiver_run);
                let t_socket = socket.clone().unwrap();
                let t_channels = Arc::clone(&channels);
                let receiver = thread::spawn(|| {
                    recv_thread(t_receiver_run, t_socket, t_channels);
                });

                Client {
                    inner: Box::new(socket),
                    serial: Arc::new(AtomicU32::new(0)),
                    receiver: Arc::new(receiver),
                    receiver_run,
                    channels,
                }
            }
        }

        impl Libvirt for Client {
            fn try_clone(&self) -> Result<Self, Error> {
                let inner = self.inner_clone()?;
                let serial = Arc::clone(&self.serial);
                let receiver = Arc::clone(&self.receiver);
                let receiver_run = Arc::clone(&self.receiver_run);
                let channels = Arc::clone(&self.channels);
                Ok(Client {
                    inner,
                    serial,
                    receiver,
                    receiver_run,
                    channels,
                })
            }

            fn fin(self) -> Result<(), Error> {
                if let Some(t) = Arc::into_inner(self.receiver) {
                    trace!("{}", stringify!(fin));
                    self.receiver_run.fetch_and(false, Ordering::SeqCst);
                    t.join().map_err(|_| Error::ReceiverStopError)?;
                }

                Ok(())
            }

            fn inner(&mut self) -> &mut Box<dyn ReadWrite> {
                &mut self.inner
            }

            fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error> {
                self.inner.clone()
            }

            fn serial_add(&mut self, value: u32) -> u32 {
                let prev = self.serial.fetch_add(value, Ordering::SeqCst);
                prev + value
            }

            fn receiver_running(&self) -> bool {
                self.receiver_run.load(Ordering::SeqCst)
            }

            fn add_channel(&mut self, serial: u32, sender: Sender<VirNetResponseRaw>) {
                let mut channels = self.channels.lock().unwrap();
                channels.insert(serial, sender);
            }

            fn remove_channel(&mut self, serial: u32) {
                let mut channels = self.channels.lock().unwrap();
                channels.remove(&serial);
            }

            fn channel_clone(&self) -> Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>> {
                Arc::clone(&self.channels)
            }
        }

        pub trait Libvirt: Send + Sized + 'static {
            fn try_clone(&self) -> Result<Self, Error>;

            fn fin(self) -> Result<(), Error>;

            fn inner(&mut self) -> &mut Box<dyn ReadWrite>;

            fn inner_clone(&self) -> Result<Box<dyn ReadWrite>, Error>;

            fn serial_add(&mut self, value: u32) -> u32;

            fn receiver_running(&self) -> bool;

            fn add_channel(&mut self, serial: u32, sender: Sender<VirNetResponseRaw>);

            fn remove_channel(&mut self, serial: u32);

            fn channel_clone(&self) -> Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>;

            #(#calls)*

            #(#msgs)*
        }

        impl VirNetStreamResponse {
            pub fn new(
                inner: Box<dyn ReadWrite>,
                channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
                receiver: Receiver<VirNetResponseRaw>,
                header: protocol::VirNetMessageHeader,
            ) -> Self {
                VirNetStreamResponse { inner, channels, receiver, header }
            }

            pub fn fin(&self) {
                let mut channels = self.channels.lock().unwrap();
                channels.remove(&self.header.serial);
            }

            pub fn download(&mut self) -> Result<Option<VirNetStream>, Error> {
                download(self)
            }

            pub fn storage_vol_upload_data(&mut self, buf: &[u8]) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_data));
                upload(self, RemoteProcedure::RemoteProcStorageVolUpload, buf)
            }

            pub fn storage_vol_upload_hole(&mut self, length: i64, flags: u32) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_hole));
                send_hole(
                    self,
                    RemoteProcedure::RemoteProcStorageVolUpload,
                    length,
                    flags,
                )
            }

            pub fn storage_vol_upload_complete(&mut self) -> Result<(), Error> {
                trace!("{}", stringify!(storage_vol_upload_complete));
                upload_completed(
                    self,
                    RemoteProcedure::RemoteProcStorageVolUpload,
                )
            }
        }

        fn call<S, D>(
            client: &mut impl Libvirt,
            procedure: RemoteProcedure,
            stream: bool,
            args: Option<S>,
        ) -> Result<VirNetResponseSet<D>, Error>
        where
            S: Serialize,
            D: DeserializeOwned,
        {
            let serial = client.serial_add(1);

            if !client.receiver_running() {
                return Err(Error::ReceiverNotStartedError);
            }

            let (tx, rx) = channel();
            client.add_channel(serial, tx);

            let socket = client.inner();

            if let Err(e) = send(
                socket,
                procedure,
                protocol::VirNetMessageType::VirNetCall,
                serial,
                protocol::VirNetMessageStatus::VirNetOk,
                args.map(|a| VirNetRequest::Data(a)),
            ) {
                client.remove_channel(serial);
                return Err(e);
            }

            let ret = read_data::<D>(stream, client.channel_clone(), &rx, serial);

            ret.map(|(header, body)| VirNetResponseSet {
                receiver: Some(rx),
                header,
                body,
            })
        }

        fn msg<D>(
            client: &mut impl Libvirt,
        ) -> Result<VirNetResponseSet<D>, Error>
        where
            D: DeserializeOwned,
        {
            let socket = client.inner();
            // TODO: event stream support.
            match recv(socket)? {
                (header, Some(VirNetResponse::Data(res))) => {
                    let set = VirNetResponseSet {
                        receiver: None,
                        header,
                        body: Some(res),
                    };
                    Ok(set)
                }
                (header, None) => {
                    let set = VirNetResponseSet {
                        receiver: None,
                        header,
                        body: None,
                    };
                    Ok(set)
                }
                _ => unreachable!(),
            }
        }

        fn download(response: &mut VirNetStreamResponse) -> Result<Option<VirNetStream>, Error> {
            let serial = response.header.serial;

            let res = response
                .receiver
                .recv_timeout(Duration::from_secs(180))
                .map_err(Error::ReceiveChannelError)?;
            if let Some(res_body_bytes) = res.body {
                match deserialize_body::<()>(&res.header, res_body_bytes) {
                    Ok(res_body) => match res_body {
                        VirNetResponse::Stream(stream) => Ok(Some(stream)),
                        _ => unreachable!(),
                    },
                    Err(e) => Err(e),
                }
            } else {
                response.channels.lock().unwrap().remove(&serial);
                Ok(None)
            }
        }

        fn upload(
            response: &mut VirNetStreamResponse,
            procedure: RemoteProcedure,
            buf: &[u8],
        ) -> Result<(), Error> {
            let bytes = VirNetStream::Raw(buf.to_vec());
            let req: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(bytes));
            send(
                &mut response.inner,
                procedure,
                protocol::VirNetMessageType::VirNetStream,
                response.header.serial,
                protocol::VirNetMessageStatus::VirNetContinue,
                req,
            )?;
            Ok(())
        }

        fn send_hole(
            response: &mut VirNetStreamResponse,
            procedure: RemoteProcedure,
            length: i64,
            flags: u32,
        ) -> Result<(), Error> {
            let hole = VirNetStream::Hole(protocol::VirNetStreamHole { length, flags });
            let args: Option<VirNetRequest<()>> = Some(VirNetRequest::Stream(hole));
            send(
                &mut response.inner,
                procedure,
                protocol::VirNetMessageType::VirNetStreamHole,
                response.header.serial,
                protocol::VirNetMessageStatus::VirNetContinue,
                args,
            )?;
            Ok(())
        }

        fn upload_completed(
            response: &mut VirNetStreamResponse,
            procedure: RemoteProcedure,
        ) -> Result<(), Error> {
            let req: Option<VirNetRequest<()>> = None;

            send(
                &mut response.inner,
                procedure,
                protocol::VirNetMessageType::VirNetStream,
                response.header.serial,
                protocol::VirNetMessageStatus::VirNetOk,
                req,
            )?;

            let (_header, _res) = read_data::<()>(
                false,
                Arc::clone(&response.channels),
                &response.receiver,
                response.header.serial,
            )?;

            Ok(())
        }

        fn send<S>(
            socket: &mut Box<dyn ReadWrite>,
            procedure: RemoteProcedure,
            req_type: protocol::VirNetMessageType,
            req_serial: u32,
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
                serial: req_serial,
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

            let mut bytes = vec![];
            bytes.extend(req_len.to_be_bytes());
            bytes.extend(req_header_bytes);
            if let Some(args_bytes) = &args_bytes {
                bytes.extend(args_bytes);
            }

            socket.write_all(&bytes).map_err(Error::SendError)?;

            Ok(bytes.len())
        }

        fn recv<D>(
            socket: &mut Box<dyn ReadWrite>,
        ) -> Result<(protocol::VirNetMessageHeader, Option<VirNetResponse<D>>), Error>
        where
            D: DeserializeOwned,
        {
            let res_len = read_pkt_len(socket)?;
            let res_header = read_res_header(socket)?;

            let body_len = res_len - 28;
            if body_len == 0 {
                return Ok((res_header, None));
            }

            let res_body_bytes = read_res_body(socket, body_len)?;
            let res_body = deserialize_body(&res_header, res_body_bytes)?;

            Ok((res_header, Some(res_body)))
        }

        fn recv_thread(
            receiver_run: Arc<AtomicBool>,
            socket: Box<dyn ReadWrite>,
            channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
        ) {
            trace!("receiver started.");
            let mut socket = socket;
            while receiver_run.load(Ordering::SeqCst) {
                match recv_raw(&mut socket) {
                    Ok((header, body_bytes)) => {
                        let serial = header.serial;

                        let raw = VirNetResponseRaw {
                            header,
                            body: body_bytes,
                        };

                        if let Some(tx) = channels.lock().unwrap().get(&serial) {
                            if let Err(e) = tx.send(raw) {
                                trace!("receiver failed to send {}.", e);
                            }
                        } else {
                            trace!("receiver not found for serial No.{}.", serial);
                        }
                    }
                    Err(Error::ReceiveError(e)) => {
                        trace!("receiver error {}.", e);
                        if e.kind() == ErrorKind::UnexpectedEof {
                            receiver_run.fetch_and(false, Ordering::SeqCst);
                        }
                    }
                    Err(e) => {
                        trace!("receiver error {}.", e);
                    }
                }
            }
            trace!("receiver stopped.");
        }

        fn recv_raw(
            socket: &mut Box<dyn ReadWrite>,
        ) -> Result<(protocol::VirNetMessageHeader, Option<Vec<u8>>), Error> {
            let res_len = read_pkt_len(socket)?;
            let res_header = read_res_header(socket)?;
            let body_len = res_len - 28;
            if body_len == 0 {
                return Ok((res_header, None));
            }
            Ok((res_header, Some(read_res_body(socket, body_len)?)))
        }

        fn read_pkt_len(socket: &mut Box<dyn ReadWrite>) -> Result<usize, Error> {
            let mut res_len_bytes = [0; 4];
            socket
                .read_exact(&mut res_len_bytes)
                .map_err(Error::ReceiveError)?;
            Ok(u32::from_be_bytes(res_len_bytes) as usize)
        }

        fn read_res_header(socket: &mut Box<dyn ReadWrite>) -> Result<protocol::VirNetMessageHeader, Error> {
            let mut res_header_bytes = [0; 24];
            socket
                .read_exact(&mut res_header_bytes)
                .map_err(Error::ReceiveError)?;
            serde_xdr::from_bytes::<protocol::VirNetMessageHeader>(&res_header_bytes)
                .map_err(Error::DeserializeError)
        }

        fn read_res_body(socket: &mut Box<dyn ReadWrite>, size: usize) -> Result<Vec<u8>, Error> {
            let mut res_body_bytes = vec![0u8; size];
            socket
                .read_exact(&mut res_body_bytes)
                .map_err(Error::ReceiveError)?;
            Ok(res_body_bytes)
        }

        fn read_data<D>(
            stream: bool,
            channels: Arc<Mutex<HashMap<u32, Sender<VirNetResponseRaw>>>>,
            rx: &Receiver<VirNetResponseRaw>,
            serial: u32,
        ) -> Result<(protocol::VirNetMessageHeader, Option<D>), Error>
        where
            D: DeserializeOwned,
        {
            let res = rx
                .recv_timeout(Duration::from_secs(180))
                .map_err(Error::ReceiveChannelError)?;

            let ret = if let Some(res_body_bytes) = res.body {
                match deserialize_body(&res.header, res_body_bytes) {
                    Ok(res_body) => match res_body {
                        VirNetResponse::Data(body) => Ok((res.header, Some(body))),
                        _ => unreachable!(),
                    },
                    Err(e) => Err(e),
                }
            } else {
                Ok((res.header, None))
            };

            if !stream {
                channels.lock().unwrap().remove(&serial);
            }

            ret
        }

        fn deserialize_body<D>(
            res_header: &protocol::VirNetMessageHeader,
            res_body_bytes: Vec<u8>,
        ) -> Result<VirNetResponse<D>, Error>
        where
            D: DeserializeOwned,
        {
            if res_header.status == protocol::VirNetMessageStatus::VirNetError {
                let res = serde_xdr::from_bytes::<protocol::VirNetMessageError>(&res_body_bytes)
                    .map_err(Error::DeserializeError)?;
                Err(Error::ProtocolError(res))
            } else {
                match res_header.r#type {
                    protocol::VirNetMessageType::VirNetReply
                    | protocol::VirNetMessageType::VirNetMessage => {
                        let data =
                            serde_xdr::from_bytes::<D>(&res_body_bytes).map_err(Error::DeserializeError)?;
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
                .find(|&m| m == &format!("Remote{method}Args"))
                .cloned();
            let method_ret = models
                .keys()
                .find(|&m| m == &format!("Remote{method}Ret"))
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
    stream: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if stream {
        return quote! { VirNetStreamResponse };
    }

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
    stream: bool,
    models: &HashMap<String, syn::ItemStruct>,
) -> TokenStream {
    if let Some(model) = model {
        let model_ident = format_ident!("{}", model);

        if wrapped || undeconstructing(model) {
            quote! {
                let res = #proc;
                Ok(res.body.unwrap())
            }
        } else {
            let model = models.get(model).unwrap();
            let fields = syn_fields_to_sig_fields(model);

            let call_stmt = quote! {
                let res = #proc;
                let res = res.body.unwrap();
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
    } else if stream {
        quote! {
            let res = #proc;
            let res = VirNetStreamResponse {
                inner: self.inner_clone()?,
                channels: self.channel_clone(),
                receiver: res.receiver.unwrap(),
                header: res.header,
            };
            Ok(res)
        }
    } else {
        quote! {
            let _res = #proc;
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

fn stream_procs(model: &str) -> bool {
    STREAM_PROCS.contains(&model)
}

fn undeconstructing(model: &str) -> bool {
    UN_DECONSTRUCTING.contains(&model)
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
