use std::io;

use io::Read;
use io::Seek;
use io::SeekFrom;

use io::BufReader;

use std::fs::File;

use bzip2::bufread::BzDecoder;

/// Gets page tags from the specified bzip2 data.
///
/// # Arguments
/// - `bzseek`: The data to be read(concatenated bzip2 files).
/// - `offset`: The number of bytes to seek(the address of the bzip2 data).
/// - `size`: The size of the bzip2 data to be read.
/// - `pages_buf`: The destination buffer.
pub fn bzseek2pages<S>(
    bzseek: &mut S,
    offset: u64,
    size: u64,
    pages_buf: &mut String,
) -> Result<(), io::Error>
where
    S: Seek + Read,
{
    bzseek.seek(SeekFrom::Start(offset))?;
    let taken = bzseek.take(size);
    let brf = BufReader::new(taken);
    let br = BzDecoder::new(brf);
    let mut brb = BufReader::new(br);
    pages_buf.clear();
    brb.read_to_string(pages_buf)?;
    Ok(())
}

/// Gets page tags from the specified bzip2 "files".
pub fn file2pages(
    bzfile: &mut File,
    offset: u64,
    size: u64,
    pages_buf: &mut String,
) -> Result<(), io::Error> {
    bzseek2pages(bzfile, offset, size, pages_buf)
}

/// Gets page tags from the specified file(concatenated bzip2 files).
pub fn filepath2pages(
    bzfilename: &str,
    offset: u64,
    size: u64,
    pages_buf: &mut String,
) -> Result<(), io::Error> {
    let mut f = File::open(bzfilename)?;
    file2pages(&mut f, offset, size, pages_buf)
}

/// Gets page tags using the computed size.
pub fn filepath2pages_compute_size(
    bzfilename: &str,
    offset: u64,
    offset_next: u64,
    pages_buf: &mut String,
) -> Result<(), io::Error> {
    let size: u64 = offset_next.saturating_sub(offset);
    filepath2pages(bzfilename, offset, size, pages_buf)
}
