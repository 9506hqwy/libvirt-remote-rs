mod error;
mod iface_list;
mod nodeinfo;
mod table_view;
mod version;

use clap::{Arg, Command};
use error::Error;
use libvirt_remote::binding::RemoteConnectOpenArgs;
use libvirt_remote::client::{Client, Libvirt};
use log::trace;
use std::net::TcpStream;
#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;
use url::Url;

fn main() -> Result<(), Error> {
    env_logger::init();

    let gargs = cmd().get_matches();

    let uri = Url::parse(gargs.value_of("connect").unwrap())?;
    let mut client = connect(uri, gargs.is_present("readonly"))?;

    let ret = match gargs.subcommand() {
        Some(("iface-list", args)) => iface_list::run(&mut client, args),
        Some(("nodeinfo", _)) => nodeinfo::run(&mut client),
        Some(("version", _)) => version::run(&mut client),
        _ => cmd().print_long_help().map_err(Error::from),
    };

    client.connect_close()?;

    ret
}

fn cmd() -> Command<'static> {
    Command::new("Libvirt Client")
        .version("0.1.0")
        .arg(
            Arg::new("connect")
                .short('c')
                .long("connect")
                .default_value(if cfg!(unix) {
                    "qemu+unix:///system"
                } else {
                    "qemu+tcp:///system"
                })
                .value_name("URI")
                .help("hypvervisor connection URI"),
        )
        .arg(
            Arg::new("readonly")
                .short('r')
                .long("readonly")
                .value_name("readonly")
                .takes_value(false)
                .help("connect readonly"),
        )
        .subcommand(iface_list::cmd())
        .subcommand(nodeinfo::cmd())
        .subcommand(version::cmd())
}

fn connect(uri: Url, readonly: bool) -> Result<Box<dyn Libvirt>, Error> {
    let schemes: Vec<&str> = uri.scheme().splitn(2, '+').collect();

    let mut client: Box<dyn Libvirt> = match schemes[1] {
        "tcp" => {
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
        "unix" if cfg!(unix) => {
            let socket = match uri.query_pairs().find(|(k, _)| k == "socket") {
                Some((_, v)) => v.into_owned(),
                _ => "/var/run/libvirt/libvirt-sock".to_string(),
            };
            trace!("connecting: {}", &socket);
            let stream = UnixStream::connect(&socket)?;
            Ok(Box::new(Client::new(stream)))
        }
        _ => Err(Error::NotSupported),
    }?;

    let name = format!("{}://{}", schemes[0], uri.path());
    trace!("connecting {} readonly={}", &name, readonly);

    let args = RemoteConnectOpenArgs {
        name: Some(name),
        flags: if readonly { 1 } else { 0 },
    };
    client.connect_open(args)?;

    Ok(client)
}
