mod cpu_stats;
mod destroy;
mod iface_list;
mod nodeinfo;
mod pool_event;
mod qemu_agent_command;
mod start;
mod version;
mod vol_download;
mod vol_upload;

use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;

pub fn app() -> Command {
    Command::new("Libvirt Client")
        .version("0.2.0")
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
                .num_args(0)
                .help("connect readonly"),
        )
        .subcommand(cpu_stats::cmd())
        .subcommand(destroy::cmd())
        .subcommand(iface_list::cmd())
        .subcommand(nodeinfo::cmd())
        .subcommand(pool_event::cmd())
        .subcommand(qemu_agent_command::cmd())
        .subcommand(start::cmd())
        .subcommand(version::cmd())
        .subcommand(vol_download::cmd())
        .subcommand(vol_upload::cmd())
}

pub fn run(
    client: &mut Box<impl Libvirt>,
    locale: &Locale,
    gargs: &ArgMatches,
) -> Result<(), Error> {
    match gargs.subcommand() {
        Some(("cpu-stats", args)) => cpu_stats::run(client, locale, args),
        Some(("destroy", args)) => destroy::run(client, locale, args),
        Some(("iface-list", args)) => iface_list::run(client, locale, args),
        Some(("nodeinfo", _)) => nodeinfo::run(client, locale),
        Some(("pool-event", args)) => pool_event::run(client, locale, args),
        Some(("qemu-agent-command", args)) => qemu_agent_command::run(client, locale, args),
        Some(("start", args)) => start::run(client, locale, args),
        Some(("version", _)) => version::run(client, locale),
        Some(("vol-download", args)) => vol_download::run(client, locale, args),
        Some(("vol-upload", args)) => vol_upload::run(client, locale, args),
        _ => app().print_long_help().map_err(Error::from),
    }
}
