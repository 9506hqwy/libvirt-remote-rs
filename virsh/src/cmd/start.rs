use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;

const DOMAIN_START_PAUSED: u32 = 1 << 0;
const DOMAIN_START_AUTODESTROY: u32 = 1 << 1;
const DOMAIN_START_BYPASS_CACHE: u32 = 1 << 2;
const DOMAIN_START_FORCE_BOOT: u32 = 1 << 3;

pub fn cmd() -> Command {
    Command::new("start")
        .arg(
            Arg::new("domain")
                .value_name("domain")
                .required(true)
                .index(1),
        )
        .arg(Arg::new("paused").long("paused").num_args(0))
        .arg(Arg::new("autodestroy").long("autodestroy").num_args(0))
        .arg(Arg::new("bypass-cache").long("bypass-cache").num_args(0))
        .arg(Arg::new("force-boot").long("force-boot").num_args(0))
}

pub fn run(
    client: &mut Box<impl Libvirt>,
    locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    let domain = args.get_one::<String>("domain").unwrap();
    let paused = args.get_flag("paused");
    let autodestroy = args.get_flag("autodestroy");
    let bypass_cache = args.get_flag("bypass-cache");
    let force_boot = args.get_flag("force-boot");

    let dom = client.domain_lookup_by_name(domain.to_string())?;

    let mut flags = 0;

    if paused {
        flags |= DOMAIN_START_PAUSED;
    }

    if autodestroy {
        flags |= DOMAIN_START_AUTODESTROY;
    }

    if bypass_cache {
        flags |= DOMAIN_START_BYPASS_CACHE;
    }

    if force_boot {
        flags |= DOMAIN_START_FORCE_BOOT;
    }

    if flags != 0 {
        client.domain_create_with_flags(dom.clone(), flags)?;
    } else {
        client.domain_create(dom.clone())?;
    }

    println!(
        "{}",
        locale.format_message("FormatDomainStarted", vec![("name", &dom.name)])
    );

    Ok(())
}
