mod cmd;
mod error;
mod kv_view;
mod locale;
mod table_view;
mod util;

use error::Error;
use libvirt_remote::binding::RemoteAuthType;
use libvirt_remote::client::{Client, Libvirt};
use log::trace;
use std::net::TcpStream;
#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;
use url::Url;

fn main() -> Result<(), Error> {
    env_logger::init();
    let locale = locale::setup()?;

    let gargs = cmd::app().get_matches();

    let uri = Url::parse(gargs.get_one::<String>("connect").unwrap())?;
    let mut client = connect(uri, gargs.get_flag("readonly"))?;

    let ret = cmd::run(&mut client, &locale, &gargs);

    client.connect_close()?;

    ret
}

fn connect(uri: Url, readonly: bool) -> Result<Box<impl Libvirt>, Error> {
    let schemes: Vec<&str> = uri.scheme().splitn(2, '+').collect();

    let mut client = match schemes[1] {
        "tcp" => connect_tcp(&uri),
        "unix" => connect_unix(&uri),
        _ => Err(Error::NotSupported),
    }?;

    let name = format!("{}://{}", schemes[0], uri.path());
    trace!("connecting {} readonly={}", &name, readonly);

    authenticate(&mut client)?;

    client.connect_open(Some(name), if readonly { 1 } else { 0 })?;

    Ok(client)
}

fn connect_tcp(uri: &Url) -> Result<Box<Client>, Error> {
    let host = format!(
        "{}:{}",
        uri.host()
            .map(|h| h.to_string())
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        uri.port().unwrap_or(16509)
    );
    trace!("connecting: {}", &host);
    let stream = TcpStream::connect(host)?;
    Ok(Box::new(Client::new(stream)))
}

#[cfg(target_family = "unix")]
fn connect_unix(uri: &Url) -> Result<Box<Client>, Error> {
    let socket = match uri.query_pairs().find(|(k, _)| k == "socket") {
        Some((_, v)) => v.into_owned(),
        _ => "/var/run/libvirt/libvirt-sock".to_string(),
    };
    trace!("connecting: {}", &socket);
    let stream = UnixStream::connect(&socket)?;
    Ok(Box::new(Client::new(stream)))
}

#[cfg(target_family = "windows")]
fn connect_unix(_: &Url) -> Result<Box<Client>, Error> {
    Err(Error::NotSupported)
}

fn authenticate(client: &mut Box<impl Libvirt>) -> Result<(), Error> {
    let auth_list = client.auth_list()?;
    if let Some(auth) = auth_list.into_iter().next() {
        match auth {
            RemoteAuthType::RemoteAuthNone => {}
            RemoteAuthType::RemoteAuthPolkit => {
                client.auth_polkit()?;
            }
            RemoteAuthType::RemoteAuthSasl => {
                return Err(Error::NotSupported);
            }
        }
    }

    Ok(())
}
