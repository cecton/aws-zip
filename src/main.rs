extern crate walkdir;
extern crate zip;
#[macro_use]
extern crate clap;
extern crate structopt;

use std::fs::File;
use std::io::Write;
use std::iter::Iterator;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{DateTime, ZipWriter};

arg_enum! {
    #[derive(Debug)]
    enum Compression {
        Store,
        Deflate,
        BZip2
    }
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// force all file permission
    #[structopt(short = "m", long = "mode")]
    mode: Option<u32>,

    /// compression method
    #[structopt(
        short = "Z",
        long = "compression-method",
        default_value = "deflate",
        raw(
            possible_values = "&Compression::variants()",
            case_insensitive = "true"
        )
    )]
    compression: Compression,

    /// Target ZIP file will be overwritten
    #[structopt(name = "ZIPFILE", parse(from_os_str))]
    zipfile: PathBuf,

    /// Input files or directories
    #[structopt(name = "FILES", parse(from_os_str), raw(required = "true"))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    std::process::exit(match run(opt) {
        Ok(0) => 0,
        Ok(errors) => errors,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    })
}

fn run(opt: Opt) -> Result<i32, std::io::Error> {
    use Compression::*;

    let mut errors = 0;
    let method = match opt.compression {
        Store => zip::CompressionMethod::Stored,
        Deflate => zip::CompressionMethod::Deflated,
        BZip2 => zip::CompressionMethod::Bzip2,
    };
    let file = File::create(&opt.zipfile)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(method)
        .last_modified_time(DateTime::from_date_and_time(1980, 1, 1, 0, 0, 0).unwrap());

    for path in opt.files {
        let walkdir = WalkDir::new(path);
        let it = walkdir.into_iter();
        for entry in it {
            match entry {
                Err(err) => {
                    eprintln!("{}", err);
                    errors += 1
                }
                Ok(entry) => {
                    let path = entry.path();

                    if path.is_file() {
                        let metadata = path.metadata().unwrap();
                        let mode = metadata.permissions().mode();
                        let options =
                            options.unix_permissions(if mode & 0o111 != 0 { 0o755 } else { 0o644 });

                        zip.start_file(path.to_str().unwrap(), options)?;
                        zip.write_all(std::fs::read(&path)?.as_slice())?;

                        println!("{}", path.display());
                    }
                }
            }
        }
    }
    zip.finish()?;

    Ok(errors)
}
