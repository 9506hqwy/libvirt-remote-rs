use super::error::Error;
use super::locale::Locale;
use chrono::Utc;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::binding::{
    RemoteConnectStoragePoolEventDeregisterAnyArgs, RemoteConnectStoragePoolEventRegisterAnyArgs,
    RemoteStoragePoolLookupByNameArgs,
};
use libvirt_remote::client::Libvirt;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static EVENTS: Lazy<HashMap<&str, i32>> = Lazy::new(|| {
    let mut e = HashMap::new();
    e.insert("lifecycle", 0);
    e.insert("refresh", 1);
    e
});

static LIFECYCLE_EVENTS: Lazy<HashMap<i32, &str>> = Lazy::new(|| {
    let mut e = HashMap::new();
    e.insert(0, "Defined");
    e.insert(1, "Undefined");
    e.insert(2, "Started");
    e.insert(3, "Stopped");
    e.insert(4, "Created");
    e.insert(5, "Deleted");
    e
});

pub fn cmd() -> Command<'static> {
    Command::new("pool-event")
        .arg(Arg::new("pool").long("pool").value_name("string"))
        .arg(Arg::new("event").long("event").value_name("string"))
        .arg(Arg::new("loop").long("loop").takes_value(false))
        .arg(Arg::new("timeout").long("timeout").value_name("number"))
        .arg(Arg::new("list").long("list").takes_value(false))
        .arg(Arg::new("timestamp").long("timestamp").takes_value(false))
}

pub fn run(
    client: &mut Box<dyn Libvirt>,
    _locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    if args.is_present("list") {
        for e in EVENTS.keys() {
            println!("{}", e);
        }
        return Ok(());
    }

    let event_name = args.value_of("event").expect("must specify --event.");
    let event_id = *EVENTS.get(event_name).expect("not found event name.");

    let pool = match args.value_of("pool") {
        Some(name) => {
            let lookup_args = RemoteStoragePoolLookupByNameArgs {
                name: name.to_string(),
            };
            let ret = client.storage_pool_lookup_by_name(lookup_args)?;
            Some(ret.pool)
        }
        _ => None,
    };

    let reg_args = RemoteConnectStoragePoolEventRegisterAnyArgs { event_id, pool };
    let callback_id = client
        .connect_storage_pool_event_register_any(reg_args)?
        .callback_id;

    // TODO: timeout
    loop {
        let msg = match event_id {
            0 => handle_lifecycle_event(client)?,
            1 => handle_refresh_event(client)?,
            _ => unreachable!(),
        };

        let time = if args.is_present("timestamp") {
            format!("{}: ", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z"))
        } else {
            "".to_string()
        };

        println!("{}{}", time, msg);

        if !args.is_present("loop") {
            break;
        }
    }

    let dereg_args = RemoteConnectStoragePoolEventDeregisterAnyArgs { callback_id };
    client.connect_storage_pool_event_deregister_any(dereg_args)?;

    Ok(())
}

fn handle_lifecycle_event(client: &mut Box<dyn Libvirt>) -> Result<String, Error> {
    let msg = client.storage_pool_event_lifecycle_msg()?;
    let id = LIFECYCLE_EVENTS.get(&msg.event).unwrap();
    Ok(format!(
        "event 'lifecycle' for storage pool {}: {}",
        msg.pool.name, id
    ))
}

fn handle_refresh_event(client: &mut Box<dyn Libvirt>) -> Result<String, Error> {
    let msg = client.storage_pool_event_refresh_msg()?;
    Ok(format!(
        "event 'refresh' for storage pool {}",
        msg.pool.name
    ))
}
