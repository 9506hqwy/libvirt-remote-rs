use crate::error::Error;
use crate::locale::Locale;
use crate::table_view::TableView;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;

const VIR_CONNECT_LIST_INTERFACES_INACTIVE: u32 = 1;
const VIR_CONNECT_LIST_INTERFACES_ACTIVE: u32 = 2;

pub fn cmd() -> Command {
    Command::new("iface-list")
        .arg(Arg::new("inactive").long("inactive").num_args(0))
        .arg(
            Arg::new("all")
                .long("all")
                .num_args(0)
                .conflicts_with("inactive"),
        )
}

pub fn run(
    client: &mut Box<impl Libvirt>,
    locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    let flags = if args.get_flag("inactive") {
        VIR_CONNECT_LIST_INTERFACES_INACTIVE
    } else if args.get_flag("all") {
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
