use super::error::Error;
use super::table_view::TableView;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::binding::{RemoteConnectListAllInterfacesArgs, RemoteInterfaceIsActiveArgs};
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

pub fn run(client: &mut Box<dyn Libvirt>, args: &ArgMatches) -> Result<(), Error> {
    let flags = if args.is_present("inactive") {
        VIR_CONNECT_LIST_INTERFACES_INACTIVE
    } else if args.is_present("all") {
        VIR_CONNECT_LIST_INTERFACES_INACTIVE | VIR_CONNECT_LIST_INTERFACES_ACTIVE
    } else {
        VIR_CONNECT_LIST_INTERFACES_ACTIVE
    };

    let args = RemoteConnectListAllInterfacesArgs {
        need_results: -1,
        flags,
    };
    let ret = client.connect_list_all_interfaces(args)?;

    let mut view = TableView::new(vec!["Name", "State", "Mac Address"]);
    for iface in ret.ifaces {
        let args = RemoteInterfaceIsActiveArgs {
            iface: iface.clone(),
        };
        let active = match client.interface_is_active(args)?.active {
            0 => "inactive",
            _ => "active",
        };

        view.add_row(vec![&iface.name, active, &iface.mac]);
    }

    view.print_table();

    Ok(())
}
