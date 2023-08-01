use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;

const DOMAIN_DESTROY_GRACEFUL: u32 = 1 << 0;

pub fn cmd() -> Command {
    Command::new("destroy")
        .arg(
            Arg::new("domain")
                .value_name("domain")
                .required(true)
                .index(1),
        )
        .arg(Arg::new("graceful").long("graceful").num_args(0))
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale, args: &ArgMatches) -> Result<(), Error> {
    let domain = args.get_one::<String>("domain").unwrap();
    let graceful = args.get_flag("graceful");

    let dom = client.domain_lookup_by_name(domain.to_string())?;

    let mut flags = 0;

    if graceful {
        flags |= DOMAIN_DESTROY_GRACEFUL;
    }

    if flags != 0 {
        client.domain_destroy_flags(dom.clone(), flags)?;
    } else {
        client.domain_destroy(dom.clone())?;
    }

    println!(
        "{}",
        locale.format_message("FormatDomainDestroyed", vec![("name", &dom.name)])
    );

    Ok(())
}
