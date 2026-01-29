use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;
use std::str::FromStr;

const DOMAIN_QEMU_AGENT_COMMAND_BLOCK: i32 = -2;
const DOMAIN_QEMU_AGENT_COMMAND_DEFAULT: i32 = -1;
const DOMAIN_QEMU_AGENT_COMMAND_NOWAIT: i32 = 0;

pub fn cmd() -> Command {
    Command::new("qemu-agent-command")
        .arg(
            Arg::new("domain")
                .value_name("domain")
                .required(true)
                .index(1),
        )
        .arg(Arg::new("cmd").value_name("cmd").required(true).index(2))
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("number")
                .value_parser(check_type::<u32>)
                .conflicts_with("block"),
        )
        .arg(
            Arg::new("async")
                .long("async")
                .num_args(0)
                .conflicts_with("timeout"),
        )
        .arg(
            Arg::new("block")
                .long("block")
                .num_args(0)
                .conflicts_with("async"),
        )
}

pub fn run(
    client: &mut Box<impl Libvirt>,
    _locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    let domain = args.get_one::<String>("domain").unwrap();
    let cmd = args.get_one::<String>("cmd").unwrap();
    let utimeout = args.get_one::<u32>("timeout");
    let async_flag = args.get_flag("async");
    let block = args.get_flag("block");

    let dom = client.domain_lookup_by_name(domain.to_string())?;

    let mut timeout = DOMAIN_QEMU_AGENT_COMMAND_DEFAULT;

    if async_flag {
        timeout = DOMAIN_QEMU_AGENT_COMMAND_NOWAIT
    } else if block {
        timeout = DOMAIN_QEMU_AGENT_COMMAND_BLOCK
    } else if let Some(t) = utimeout {
        timeout = *t as i32;
    }

    let flags = 0;
    let output = client.domain_agent_command(dom.clone(), cmd.clone(), timeout, flags)?;

    println!("{}", output.unwrap_or_default());

    Ok(())
}

fn check_type<T>(value: &str) -> Result<T, String>
where
    T: FromStr,
{
    Ok(value.parse::<T>().map_err(|_| value)?)
}
