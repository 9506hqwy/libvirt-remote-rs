use crate::error::Error;
use crate::locale::Locale;
use clap::Command;
use libvirt_remote::client::Libvirt;
use log::error;

pub fn cmd() -> Command {
    Command::new("version")
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale) -> Result<(), Error> {
    let hv_type = match client.connect_get_type() {
        Ok(ret) => Some(ret),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let hv_ver = match client.connect_get_version() {
        Ok(ret) => Some(version_string(ret)),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let lib_ver = match client.connect_get_lib_version() {
        Ok(ret) => Some(version_string(ret)),
        Err(e) => {
            error!("{}", e);
            None
        }
    };

    if lib_ver.is_some() {
        println!(
            "{}",
            locale.format_message(
                "FormatCompiledLibrary",
                vec![("version", lib_ver.as_ref().unwrap())]
            ),
        );
    }

    if lib_ver.is_some() {
        println!(
            "{}",
            locale.format_message(
                "FormatUsingLibrary",
                vec![("version", lib_ver.as_ref().unwrap())]
            )
        );
    }

    if hv_type.is_some() && lib_ver.is_some() {
        println!(
            "{}",
            locale.format_message(
                "FormatUsingAPI",
                vec![
                    ("type", hv_type.as_ref().unwrap()),
                    ("version", lib_ver.as_ref().unwrap())
                ]
            ),
        );
    }

    if hv_type.is_some() && hv_ver.is_some() {
        println!(
            "{}",
            locale.format_message(
                "FormatRunningHypervisor",
                vec![
                    ("type", hv_type.as_ref().unwrap()),
                    ("version", hv_ver.as_ref().unwrap())
                ]
            ),
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
