use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::SaveloadError as Error;

/// Compress a byte slice using zlib.
pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(bytes).map_err(Error::from)?;
    e.finish().map_err(Error::from)
}

/// Decompress a byte slice using zlib.
pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut d = ZlibDecoder::new(bytes);
    let mut data = Vec::new();
    d.read_to_end(&mut data).map_err(Error::from)?;
    Ok(data)
}
