pub mod db;
pub mod recorder;
pub mod tributes;

use std::{fs::File, io::Read, path::Path};

use bitvec::{slice::BitSlice, vec::BitVec};
use memmap2::{Mmap, MmapOptions};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tsz_compress::prelude::*;
use zstd::dict::DecoderDictionary;

use crate::VCRResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct DataHeader {
    pub id: [u8; 16],
    pub times: Vec<i64>,
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub offset: u32,
    pub checkpoint_every: usize,
    pub checkpoint_positions: Vec<usize>, // positions in the decompressed byte slice at which checkpoints happen
}

impl DataHeader {
    // doesn't check wheter time is before at
    pub fn find_time_unchecked(&self, at: i64) -> usize {
        match self.times.binary_search(&at) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        }
    }

    pub fn find_time(&self, at: i64) -> Option<usize> {
        let index = match self.times.binary_search(&at) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        if self.times[index] > at {
            None
        } else {
            Some(index)
        }
    }

    pub fn encode(self) -> CompressedDataHeader {
        CompressedDataHeader {
            id: self.id,
            compressed_len: self.compressed_len,
            decompressed_len: self.decompressed_len,
            offset: self.offset,
            checkpoint_every: self.checkpoint_every,
            times: compress_rows(self.times),
            checkpoint_positions: compress_rows(self.checkpoint_positions),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CompressedDataHeader {
    pub id: [u8; 16],
    pub times: BitVec<u8>,
    pub compressed_len: u32,
    pub decompressed_len: u32,
    pub offset: u32,
    pub checkpoint_every: usize,
    pub checkpoint_positions: BitVec<u8>,
}

impl CompressedDataHeader {
    pub fn decode(self) -> DataHeader {
        DataHeader {
            id: self.id,
            compressed_len: self.compressed_len,
            decompressed_len: self.decompressed_len,
            offset: self.offset,
            checkpoint_every: self.checkpoint_every,
            times: decompress_rows(&self.times),
            checkpoint_positions: decompress_rows(&self.checkpoint_positions),
        }
    }
}

#[derive(Compressible, Decompressible, DeltaEncodable, Clone, Copy, PartialEq)]
struct SingleValueRow {
    value: i64,
}

impl From<usize> for SingleValueRow {
    fn from(value: usize) -> Self {
        SingleValueRow {
            value: value.try_into().unwrap(),
        }
    }
}

impl From<i64> for SingleValueRow {
    fn from(value: i64) -> Self {
        SingleValueRow { value }
    }
}

impl From<SingleValueRow> for usize {
    fn from(val: SingleValueRow) -> Self {
        val.value.try_into().unwrap()
    }
}

impl From<SingleValueRow> for i64 {
    fn from(val: SingleValueRow) -> Self {
        val.value
    }
}

fn compress_rows<T: Into<SingleValueRow>>(rows: impl IntoIterator<Item = T>) -> BitVec<u8> {
    let mut compressor = Compressor::new(10000);
    for row in rows {
        compressor.compress(row.into());
    }

    compressor.finish()
}

fn decompress_rows<T: From<SingleValueRow>>(rows: &BitSlice<u8>) -> Vec<T> {
    let mut out = Vec::new();
    let mut decompressor = Decompressor::new(rows);
    for row in decompressor.decompress::<SingleValueRow>() {
        out.push(row.unwrap().into())
    }

    out
}

pub struct TapeComponents<T: DeserializeOwned> {
    pub dict: Option<DecoderDictionary<'static>>,
    pub header: T,
    pub store: Mmap,
}

impl<T: DeserializeOwned> TapeComponents<T> {
    pub fn split(path: impl AsRef<Path>) -> VCRResult<TapeComponents<T>> {
        let mut file = File::open(path)?;
        let mut len_bytes: [u8; 8] = [0; 8];
        file.read_exact(&mut len_bytes)?;

        let dict_len = u64::from_le_bytes(len_bytes) as usize;

        let dict = if dict_len > 0 {
            let mut dict = vec![0u8; dict_len];
            file.read_exact(&mut dict)?;
            Some(DecoderDictionary::copy(&dict[..]))
        } else {
            None
        };

        file.read_exact(&mut len_bytes)?;
        let header_len = u64::from_le_bytes(len_bytes) as usize;

        let mut header_bytes = vec![0u8; header_len];
        file.read_exact(&mut header_bytes)?;

        let header: T = rmp_serde::from_read(zstd::Decoder::new(&header_bytes[..])?)?;

        let total_len = file.metadata()?.len() as usize;

        let inner = unsafe {
            MmapOptions::new()
                .offset((dict_len + header_len + 16) as u64)
                .len(total_len - (dict_len + header_len + 16))
                .map(&file)?
        };

        Ok(TapeComponents {
            dict,
            header,
            store: inner,
        })
    }
}
