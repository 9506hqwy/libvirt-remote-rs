use crate::error::Error;
use crate::locale::Locale;
use chrono::Utc;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;
use std::collections::HashMap;
use std::sync::OnceLock;

static EVENTS: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();

fn init_events() -> HashMap<&'static str, i32> {
    let mut e = HashMap::new();
    e.insert("lifecycle", 0);
    e.insert("refresh", 1);
    e
}

static LIFECYCLE_EVENTS: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();

fn init_lifecycle_events() -> HashMap<i32, &'static str> {
    let mut e = HashMap::new();
    e.insert(0, "Defined");
    e.insert(1, "Undefined");
    e.insert(2, "Started");
    e.insert(3, "Stopped");
    e.insert(4, "Created");
    e.insert(5, "Deleted");
    e
}

pub fn cmd() -> Command {
    Command::new("pool-event")
        .arg(Arg::new("pool").long("pool").value_name("string"))
        .arg(Arg::new("event").long("event").value_name("string"))
        .arg(Arg::new("loop").long("loop").num_args(0))
        .arg(Arg::new("timeout").long("timeout").value_name("number"))
        .arg(Arg::new("list").long("list").num_args(0))
        .arg(Arg::new("timestamp").long("timestamp").num_args(0))
}

pub fn run(
    client: &mut Box<dyn Libvirt>,
    _locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    if args.get_flag("list") {
        for e in EVENTS.get_or_init(init_events).keys() {
            println!("{e}");
        }
        return Ok(());
    }

    let event_name = args
        .get_one::<String>("event")
        .expect("must specify --event.");
    let event_id = *EVENTS
        .get_or_init(init_events)
        .get(event_name.as_str())
        .expect("not found event name.");

    let pool = match args.get_one::<String>("pool") {
        Some(name) => {
            let pool = client.storage_pool_lookup_by_name(name.to_string())?;
            Some(pool)
        }
        _ => None,
    };

    let callback_id = client.connect_storage_pool_event_register_any(event_id, pool)?;

    // TODO: timeout
    loop {
        let msg = match event_id {
            0 => handle_lifecycle_event(client)?,
            1 => handle_refresh_event(client)?,
            _ => unreachable!(),
        };

        let time = if args.get_flag("timestamp") {
            format!("{}: ", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z"))
        } else {
            "".to_string()
        };

        println!("{time}{msg}");

        if !args.get_flag("loop") {
            break;
        }
    }

    client.connect_storage_pool_event_deregister_any(callback_id)?;

    Ok(())
}

fn handle_lifecycle_event(client: &mut Box<dyn Libvirt>) -> Result<String, Error> {
    let (_, pool, event, _) = client.storage_pool_event_lifecycle_msg()?;
    let id = LIFECYCLE_EVENTS
        .get_or_init(init_lifecycle_events)
        .get(&event)
        .unwrap();
    Ok(format!(
        "event 'lifecycle' for storage pool {}: {}",
        pool.name, id
    ))
}

fn handle_refresh_event(client: &mut Box<dyn Libvirt>) -> Result<String, Error> {
    let (_, pool) = client.storage_pool_event_refresh_msg()?;
    Ok(format!("event 'refresh' for storage pool {}", pool.name))
}
