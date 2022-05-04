use super::error::Error;
use super::locale::Locale;
use clap::Command;
use libvirt_remote::client::Libvirt;

pub fn cmd() -> Command<'static> {
    Command::new("nodeinfo")
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale) -> Result<(), Error> {
    let info = client.node_get_info()?;

    println!(
        "{}  {}",
        locale.get_message("LabelCpuModel"),
        to_utf8_str(&info.model)?
    );
    println!("{}  {}", locale.get_message("LabelCpuNum"), info.cpus);
    println!("{}  {} MHz", locale.get_message("LabelCpuFreq"), info.mhz);
    println!(
        "{}  {}",
        locale.get_message("LabelCpuSocketNum"),
        info.sockets
    );
    println!("{}  {}", locale.get_message("LabelCpuCoreNum"), info.cores);
    println!(
        "{}  {}",
        locale.get_message("LabelCpuThreadNum"),
        info.threads
    );
    println!(
        "{}  {}",
        locale.get_message("LabelMemoryNumaCellNum"),
        info.nodes
    );
    println!(
        "{}  {} KiB",
        locale.get_message("LabelMemorySize"),
        info.memory
    );
    println!();

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
