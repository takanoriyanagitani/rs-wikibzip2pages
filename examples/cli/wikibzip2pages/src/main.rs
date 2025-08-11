use std::io;
use std::io::ErrorKind;

use std::io::Write;

use std::process::ExitCode;

use std::env;

use rs_wikibzip2pages::filepath2pages;

fn env2size() -> Result<u64, io::Error> {
    let raw = env::var("BZIP2_SIZE").map_err(|_| {
        io::Error::new(
            ErrorKind::NotFound,
            "environment variable BZIP2_SIZE is not set",
        )
    })?;
    str::parse(&raw).map_err(io::Error::other)
}

fn env2bzfilename() -> Result<String, io::Error> {
    env::var("BZ_FILENAME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "BZ_FILENAME not set"))
}

fn env2offset() -> Result<u64, io::Error> {
    let val = env::var("BZ_OFFSET")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "BZ_OFFSET not set"))?;
    str::parse(&val).map_err(io::Error::other)
}

fn sub() -> Result<(), io::Error> {
    let bzname: String = env2bzfilename()?;

    let ofst: u64 = env2offset()?;
    let size: u64 = env2size()?;

    let mut buf: String = String::new();

    filepath2pages(&bzname, ofst, size, &mut buf)?;

    let o = io::stdout();
    let mut ol = o.lock();

    ol.write_all(buf.as_bytes())?;
    ol.flush()?;

    Ok(())
}

fn main() -> ExitCode {
    match sub() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
