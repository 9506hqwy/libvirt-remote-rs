use super::error::Error;
use clap::Command;
use libvirt_remote::client::Libvirt;

pub fn cmd() -> Command<'static> {
    Command::new("nodeinfo")
}

pub fn run(client: &mut Box<dyn Libvirt>) -> Result<(), Error> {
    let info = client.node_get_info()?;

    println!("CPU model:           {}", to_utf8_str(&info.model)?);
    println!("CPU(s):              {}", info.cpus);
    println!("CPU frequency:       {} MHz", info.mhz);
    println!("CPU socket(s):       {}", info.sockets);
    println!("Core(s) per socket:  {}", info.cores);
    println!("Thread(s) per core:  {}", info.threads);
    println!("NUMA cell(s):        {}", info.nodes);
    println!("Memory size:         {} KiB", info.memory);
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
