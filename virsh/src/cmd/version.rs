use crate::error::Error;
use crate::locale::Locale;
use clap::Command;
use libvirt_remote::client::Libvirt;
use log::error;
use std::sync::mpsc::channel;
use std::thread;

pub fn cmd() -> Command {
    Command::new("version")
}

pub fn run(client: &mut Box<impl Libvirt>, locale: &Locale) -> Result<(), Error> {
    let (hv_type_tx, hv_type_rx) = channel();
    let (hv_ver_tx, hv_ver_rx) = channel();
    let (lib_ver_tx, lib_ver_rx) = channel();

    thread::scope(|s| {
        let mut c1 = client.try_clone().unwrap();
        s.spawn(move || {
            let hv_type = match c1.connect_get_type() {
                Ok(ret) => Some(ret),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            };
            hv_type_tx.send(hv_type).unwrap();
            c1.fin().unwrap();
        });

        let mut c2 = client.try_clone().unwrap();
        s.spawn(move || {
            let hv_ver = match c2.connect_get_version() {
                Ok(ret) => Some(version_string(ret)),
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            };
            hv_ver_tx.send(hv_ver).unwrap();
            c2.fin().unwrap();
        });

        let mut c3 = client.try_clone().unwrap();
        s.spawn(move || {
            let lib_ver = match c3.connect_get_lib_version() {
                Ok(ret) => Some(version_string(ret)),
                Err(e) => {
                    error!("{e}");
                    None
                }
            };
            lib_ver_tx.send(lib_ver).unwrap();
            c3.fin().unwrap();
        });
    });

    let hv_type: Option<String> = hv_type_rx.recv().unwrap();
    let hv_ver: Option<String> = hv_ver_rx.recv().unwrap();
    let lib_ver: Option<String> = lib_ver_rx.recv().unwrap();

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
    format!("{major}.{minor}.{release}")
}
