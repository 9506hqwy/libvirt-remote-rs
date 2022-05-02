use super::error::Error;
use clap::Command;
use libvirt_remote::client::Libvirt;
use log::error;

pub fn cmd() -> Command<'static> {
    Command::new("version")
}

pub fn run(client: &mut Box<dyn Libvirt>) -> Result<(), Error> {
    let hv_type = match client.connect_get_type() {
        Ok(ret) => Some(ret.r#type),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let hv_ver = match client.connect_get_version() {
        Ok(ret) => Some(version_string(ret.hv_ver)),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let lib_ver = match client.connect_get_lib_version() {
        Ok(ret) => Some(version_string(ret.lib_ver)),
        Err(e) => {
            error!("{}", e);
            None
        }
    };

    if lib_ver.is_some() {
        println!(
            "Compiled against library: libvirt {}",
            lib_ver.as_ref().unwrap()
        );
    }

    if lib_ver.is_some() {
        println!("Using library: libvirt {}", lib_ver.as_ref().unwrap());
    }

    if hv_type.is_some() && lib_ver.is_some() {
        println!(
            "Using API: {} {}",
            hv_type.as_ref().unwrap(),
            lib_ver.as_ref().unwrap()
        );
    }

    if hv_type.is_some() && hv_ver.is_some() {
        println!(
            "Running hypervisor: {} {}",
            hv_type.as_ref().unwrap(),
            hv_ver.as_ref().unwrap()
        );
    }

    println!();

    Ok(())
}

fn version_string(version: u64) -> String {
    let major = (version / 1000000) % 1000;
    let minor = (version / 1000) % 1000;
    let release = version % 1000;
    return format!("{}.{}.{}", major, minor, release);
}
