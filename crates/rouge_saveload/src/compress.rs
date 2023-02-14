use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

pub fn compress(bytes: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(bytes).expect("failed to write");
    e.finish().expect("failed to compress")
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut d = ZlibDecoder::new(bytes);
    let mut data = Vec::new();
    d.read_to_end(&mut data).unwrap();
    data
}
