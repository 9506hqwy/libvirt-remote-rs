use crate::error::Error;
use crate::kv_view::KeyValueView;
use crate::locale::Locale;
use clap::Command;
use libvirt_remote::client::Libvirt;

pub fn cmd() -> Command<'static> {
    Command::new("nodeinfo")
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale) -> Result<(), Error> {
    let info = client.node_get_info()?;

    let mut view = KeyValueView::default();

    view.add_row(
        &locale.get_message("LabelCpuModel"),
        &to_utf8_str(&info.model)?,
    );
    view.add_row(&locale.get_message("LabelCpuNum"), info.cpus);
    view.add_row(
        &locale.get_message("LabelCpuFreq"),
        format!("{} Mhz", info.mhz),
    );
    view.add_row(&locale.get_message("LabelCpuSocketNum"), info.sockets);
    view.add_row(&locale.get_message("LabelCpuCoreNum"), info.cores);
    view.add_row(&locale.get_message("LabelCpuThreadNum"), info.threads);
    view.add_row(&locale.get_message("LabelMemoryNumaCellNum"), info.nodes);
    view.add_row(
        &locale.get_message("LabelMemorySize"),
        format!("{} KiB", info.memory),
    );

    view.print_kv();

    Ok(())
}

fn to_utf8_str(value: &[i8]) -> Result<String, Error> {
    let bytes: Vec<u8> = value
        .iter()
        .map(|&c| c as u8)
        .into_iter()
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf8(bytes).map_err(Error::from)
}
