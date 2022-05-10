use super::error::Error;
use super::locale::Locale;
use super::table_view::TableView;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;

const VIR_CONNECT_LIST_INTERFACES_INACTIVE: u32 = 1;
const VIR_CONNECT_LIST_INTERFACES_ACTIVE: u32 = 2;

pub fn cmd() -> Command<'static> {
    Command::new("iface-list")
        .arg(Arg::new("inactive").long("inactive").takes_value(false))
        .arg(
            Arg::new("all")
                .long("all")
                .takes_value(false)
                .conflicts_with("inactive"),
        )
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale, args: &ArgMatches) -> Result<(), Error> {
    let flags = if args.is_present("inactive") {
        VIR_CONNECT_LIST_INTERFACES_INACTIVE
    } else if args.is_present("all") {
        VIR_CONNECT_LIST_INTERFACES_INACTIVE | VIR_CONNECT_LIST_INTERFACES_ACTIVE
    } else {
        VIR_CONNECT_LIST_INTERFACES_ACTIVE
    };

    let (ifaces, _) = client.connect_list_all_interfaces(-1, flags)?;

    let mut view = TableView::new(vec![
        &locale.get_message("Name"),
        &locale.get_message("State"),
        &locale.get_message("MacAddress"),
    ]);
    for iface in ifaces {
        let active = match client.interface_is_active(iface.clone())? {
            0 => locale.get_message("Inactive"),
            _ => locale.get_message("Active"),
        };

        view.add_row(vec![&iface.name, &active, &iface.mac]);
    }

    view.print_table();

    Ok(())
}
