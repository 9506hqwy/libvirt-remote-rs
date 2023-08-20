use crate::error::Error;
use crate::locale::Locale;
use clap::{Arg, ArgMatches, Command};
use libvirt_remote::client::Libvirt;
use std::fs;
use std::io::{BufReader, Read, Seek, SeekFrom};
#[cfg(target_family = "unix")]
use std::os::unix::io::AsRawFd;

const MESSAGE_LEGACY_PAYLOAD_MAX: usize = 262120;

enum SparseType {
    Data,
    Hole,
}

pub fn cmd() -> Command {
    Command::new("vol-upload")
        .arg(Arg::new("vol").value_name("vol").index(1))
        .arg(Arg::new("file").value_name("file").index(2))
        .arg(Arg::new("pool").long("pool").value_name("string"))
        .arg(Arg::new("offset").long("offset").value_name("number"))
        .arg(Arg::new("length").long("length").value_name("number"))
        .arg(Arg::new("sparse").long("sparse").num_args(0))
}

pub fn run(
    client: &mut Box<dyn Libvirt>,
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

    client.storage_vol_upload(volume, offset, length, flags)?;

    let mut f = fs::OpenOptions::new().read(true).open(file)?;
    f.seek(SeekFrom::Start(offset))?;

    let remain = if length == 0 {
        f.metadata()?.len() as usize
    } else {
        length as usize
    };

    if flags == 0 {
        let mut reader = BufReader::new(f);
        upload_data(client, &mut reader, remain)?;
    } else {
        let ranges = parse_sparse_file(&mut f, remain);
        f.seek(SeekFrom::Start(offset))?;

        for (sparse_type, length) in ranges {
            match sparse_type {
                SparseType::Hole => {
                    client.storage_vol_upload_hole(length as i64, 0)?;
                    f.seek(SeekFrom::Current(length as i64))?;
                }
                SparseType::Data => {
                    upload_data(client, &mut f, length)?;
                }
            }
        }
    }

    client.storage_vol_upload_complete()?;

    Ok(())
}

fn upload_data(
    client: &mut Box<dyn Libvirt>,
    reader: &mut impl Read,
    length: usize,
) -> Result<(), Error> {
    let mut remain = length;

    while remain > 0 {
        let buf_size = if remain > MESSAGE_LEGACY_PAYLOAD_MAX {
            MESSAGE_LEGACY_PAYLOAD_MAX
        } else {
            remain
        };

        let mut buf = vec![0; buf_size];
        let size = reader.read(&mut buf)?;
        if size == 0 {
            break;
        }

        remain -= size;

        client.storage_vol_upload_data(&buf[..size])?;
    }

    Ok(())
}

#[cfg(target_family = "windows")]
fn parse_sparse_file(_file: &mut fs::File, _length: usize) -> Vec<(SparseType, usize)> {
    unimplemented!();
}

#[cfg(target_family = "unix")]
fn parse_sparse_file(file: &mut fs::File, length: usize) -> Vec<(SparseType, usize)> {
    let mut ranges = vec![];

    let mut remain = length;
    while remain > 0 {
        let current = file.stream_position().unwrap() as usize;
        if let Some(next_hole_offset) = seek_next_hole(file, current).unwrap() {
            match next_hole_offset - current {
                0 => match seek_next_data(file, current).unwrap() {
                    Some(next_data_offset) => {
                        let hole_size = next_data_offset - current;
                        let hole_size = if hole_size > remain {
                            remain
                        } else {
                            hole_size
                        };
                        ranges.push((SparseType::Hole, hole_size));
                        remain -= hole_size;
                    }
                    _ => {
                        // EOF
                        if remain > 0 {
                            ranges.push((SparseType::Hole, remain));
                            remain = 0;
                        }
                    }
                },
                n => {
                    let data_size = if n > remain { remain } else { n };
                    ranges.push((SparseType::Data, data_size));
                    remain -= data_size;
                }
            }
        }
    }

    ranges
}

#[cfg(target_family = "unix")]
fn seek_next_hole(f: &fs::File, current: usize) -> Result<Option<usize>, i32> {
    unsafe {
        let fd = f.as_raw_fd();
        let offset = libc::lseek64(fd, current as i64, libc::SEEK_HOLE);
        errno(offset)
    }
}

#[cfg(target_family = "unix")]
fn seek_next_data(f: &fs::File, current: usize) -> Result<Option<usize>, i32> {
    unsafe {
        let fd = f.as_raw_fd();
        let offset = libc::lseek64(fd, current as i64, libc::SEEK_DATA);
        errno(offset)
    }
}

#[cfg(target_family = "unix")]
fn errno(offset: i64) -> Result<Option<usize>, i32> {
    if offset == -1 {
        let errno = unsafe { *libc::__errno_location() };
        if errno == libc::ENXIO {
            Ok(None)
        } else {
            Err(errno)
        }
    } else {
        Ok(Some(offset as usize))
    }
}
