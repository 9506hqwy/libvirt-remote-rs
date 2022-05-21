use super::error::Error;
use super::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::binding::{RemoteTypedParam, RemoteTypedParamValue};
use libvirt_remote::client::Libvirt;
use std::str::FromStr;

pub fn cmd() -> Command<'static> {
    Command::new("cpu-stats")
        .arg(Arg::new("domain").value_name("domain").required(true))
        .arg(Arg::new("total").long("total").takes_value(false))
        .arg(
            Arg::new("start")
                .long("start")
                .value_name("number")
                .validator(check_type::<i32>),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .value_name("number")
                .validator(check_type::<u32>),
        )
}

pub fn run(client: &mut Box<dyn Libvirt>, locale: &Locale, args: &ArgMatches) -> Result<(), Error> {
    let domain = args.value_of("domain").unwrap();
    let show_total = args.is_present("total");
    let start = args.value_of("start").unwrap_or("0").parse().unwrap();
    let count = args
        .value_of("count")
        .unwrap_or("-1")
        .parse::<i32>()
        .unwrap();

    let dom = client.domain_lookup_by_name(domain.to_string())?;

    let (_, max_cpu_num) = client.domain_get_cpu_stats(dom.clone(), 0, 0, 0, 0)?;
    if args.is_present("start") && max_cpu_num <= start {
        return Err(Error::Arg(format!("start={}", max_cpu_num)));
    }

    let count = if count < 0 || max_cpu_num < count {
        (max_cpu_num - start) as u32
    } else {
        count as u32
    };

    let (_, nparams) = client.domain_get_cpu_stats(dom.clone(), 0, 0, 1, 0)?;
    let nparams = nparams as u32;
    if nparams > 0 {
        let (params, _) = client.domain_get_cpu_stats(dom.clone(), nparams, start, count, 0)?;
        for ncpu in 0..(count as usize) {
            println!("CPU{}:", ncpu);

            for nparam in 0..(nparams as usize) {
                let param = &params[ncpu + nparam];
                print_ulong(param);
            }
        }
    }

    if show_total {
        let (_, nparams) = client.domain_get_cpu_stats(dom.clone(), 0, -1, 1, 0)?;
        let nparams = nparams as u32;
        let (params, _) = client.domain_get_cpu_stats(dom, nparams, -1, 1, 0)?;
        println!("{}", locale.get_message("LabelTotal"));
        for p in params {
            print_ulong(&p)
        }
    }

    Ok(())
}

fn check_type<T>(value: &str) -> Result<(), String>
where
    T: FromStr,
{
    value.parse::<T>().map_err(|_| value)?;
    Ok(())
}

fn print_ulong(param: &RemoteTypedParam) {
    print!("\t{:<12}", param.field);

    match param.value {
        RemoteTypedParamValue::VirTypedParamUllong(v) => {
            let s = v / 1_000_000_000;
            let n = v % 1_000_000_000;
            println!("{:>9}.{:>09} seconds", s, n);
        }
        _ => unreachable!(),
    }
}
