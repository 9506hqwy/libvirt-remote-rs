use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::{Libvirt, VirNetStream};
use log::trace;
use std::fs;
use std::io::{BufWriter, Seek, SeekFrom, Write};

pub fn cmd() -> Command {
    Command::new("vol-download")
        .arg(Arg::new("vol").value_name("vol").index(1))
        .arg(Arg::new("file").value_name("file").index(2))
        .arg(Arg::new("pool").long("pool").value_name("string"))
        .arg(Arg::new("offset").long("offset").value_name("number"))
        .arg(Arg::new("length").long("length").value_name("number"))
        .arg(Arg::new("sparse").long("sparse").num_args(0))
}

pub fn run(
    client: &mut Box<impl Libvirt>,
    _locale: &Locale,
    args: &ArgMatches,
) -> Result<(), Error> {
    let vol = args.get_one::<String>("vol").expect("must specify vol.");
    let file = args.get_one::<String>("file").expect("must specify file");

    let volume = match args.get_one::<String>("pool") {
        Some(pool) => {
            let pool = client.storage_pool_lookup_by_name(pool.to_string())?;
            client.storage_vol_lookup_by_name(pool, vol.to_string())?
        }
        _ => client.storage_vol_lookup_by_path(vol.to_string())?,
    };

    let offset = args
        .get_one::<String>("offset")
        .unwrap_or(&"0".to_string())
        .parse()
        .expect("offset is number.");
    let length = args
        .get_one::<String>("length")
        .unwrap_or(&"0".to_string())
        .parse()
        .expect("length is numner.");
    let flags = if args.get_flag("sparse") { 1 } else { 0 };

    client.storage_vol_download(volume, offset, length, flags)?;

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)?;
    f.seek(SeekFrom::Start(offset))?;

    let mut writer = BufWriter::new(f);
    while let Some(stream) = client.download()? {
        match stream {
            VirNetStream::Hole(hole) => {
                let length = hole.length as usize;
                trace!("hole size: {length}");
                let mut size: usize = 0;
                while size < length {
                    let bufsize = if length - size > 8192 {
                        8192
                    } else {
                        length - size
                    };
                    let buf = vec![0; bufsize];
                    size += writer.write(&buf)?;
                }
            }
            VirNetStream::Raw(buf) => {
                trace!("buffer size: {}", buf.len());
                writer.write_all(&buf)?;
            }
        }
    }

    Ok(())
}
