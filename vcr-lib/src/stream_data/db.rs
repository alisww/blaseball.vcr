use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
    marker::PhantomData,
    ops::Range,
    sync::Arc,
    time::Duration,
};

use arrayref::array_refs;
use memmap2::Mmap;
use moka::sync::Cache;
use serde::{Deserialize, Serialize};
use tsz_compress::prelude::BitBufferSlice;
use uuid::Uuid;
use xxhash_rust::xxh3;
use zstd::bulk::Decompressor;

use crate::{
    db_manager::{self, DatabaseManager},
    timestamp_to_millis,
    vhs::decompress_rows,
    ChroniclerEntity, EntityDatabase, RangeTuple, VCRResult,
};

use super::{thisidisstaticyo::StreamDataWrapper, PackedStreamComponent, StreamComponent};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StreamEntityWrapper<I: StreamComponent> {
    pub value: I,
}

pub struct StreamBatch<'a, I: StreamComponent, P: PackedStreamComponent> {
    pub times: &'a [i64],
    pub data: Vec<I::Packed>,
    _spooky: PhantomData<P>,
}

impl<'a, I: StreamComponent<Packed = P>, P: PackedStreamComponent<Unpacked = I>>
    StreamBatch<'a, I, P>
{
    pub fn index_by_time(&self, at: i64) -> Option<usize> {
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

    pub fn unpack_one(
        &self,
        index: usize,
        db: &DatabaseManager,
    ) -> VCRResult<ChroniclerEntity<StreamEntityWrapper<I>>> {
        self.data[index]
            .unpack(self.times[index], db)
            .map(|data| ChroniclerEntity {
                entity_id: [0u8; 16],
                valid_from: self.times[index],
                data: StreamEntityWrapper { value: data },
            })
    }

    pub fn unpack_many(
        &'a self,
        start_index: usize,
        end_index: usize,
        db: &'a DatabaseManager,
    ) -> impl Iterator<Item = VCRResult<ChroniclerEntity<StreamEntityWrapper<I>>>> + 'a {
        self.times[start_index..end_index]
            .iter()
            .zip(self.data[start_index..end_index].iter())
            .map(|(time, data)| {
                data.unpack(*time, db).map(|data| ChroniclerEntity {
                    entity_id: [0u8; 16],
                    valid_from: *time,
                    data: StreamEntityWrapper { value: data },
                })
            })
    }
}

pub struct StreamBatchHeader {
    pub times_len: usize,
    pub times_bits_len: usize,
    pub data_compressed_len: usize,
    pub data_uncompressed_len: usize,
}

impl StreamBatchHeader {
    pub fn encode(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&(self.times_len as u32).to_le_bytes());
        out[4..8].copy_from_slice(&(self.times_bits_len as u32).to_le_bytes());
        out[8..12].copy_from_slice(&(self.data_compressed_len as u32).to_le_bytes());
        out[12..].copy_from_slice(&(self.data_uncompressed_len as u32).to_le_bytes());
        out
    }

    pub fn decode(data: &[u8; 16]) -> StreamBatchHeader {
        let (times_len, times_bits_len, data_compressed_len, data_uncompressed_len) =
            array_refs![data, 4, 4, 4, 4];

        StreamBatchHeader {
            times_len: u32::from_le_bytes(*times_len) as usize,
            times_bits_len: u32::from_le_bytes(*times_bits_len) as usize,
            data_compressed_len: u32::from_le_bytes(*data_compressed_len) as usize,
            data_uncompressed_len: u32::from_le_bytes(*data_uncompressed_len) as usize,
        }
    }
}

struct StreamBatchDescriptor {
    times: Vec<i64>,
    data_offset: u32,
    compressed_len: u32,
    decompressed_len: u32,
}

pub struct StreamDatabase<I: StreamComponent<Packed = P>, P: PackedStreamComponent<Unpacked = I>> {
    store: Mmap,
    cache: Cache<RangeTuple, Arc<Vec<u8>>, xxh3::Xxh3Builder>,
    seek_table: Vec<(i64, StreamBatchDescriptor)>, // (time, offset)
    db_manager: Arc<DatabaseManager>,
    _spooky: PhantomData<(I, P)>,
}

impl<I: StreamComponent<Packed = P>, P: PackedStreamComponent<Unpacked = I>> StreamDatabase<I, P> {
    pub fn initialize(
        file: File,
        db_manager: Arc<DatabaseManager>,
    ) -> VCRResult<StreamDatabase<I, P>> {
        let mut seek_table = Vec::with_capacity(2_000_000);

        let mut reader = BufReader::new(file);

        let mut header_buffer = [0u8; 16];
        let mut times_buffer = Vec::new();

        while reader.has_data_left()? {
            reader.read_exact(&mut header_buffer)?;
            let header = StreamBatchHeader::decode(&header_buffer);
            times_buffer.resize(header.times_len, 0);
            reader.read_exact(&mut times_buffer)?;
            let time_bits = BitBufferSlice::from_slice(&times_buffer);
            let time_bits = time_bits.split_at(header.times_bits_len).0;

            let times: Vec<i64> = decompress_rows(&time_bits);
            let offset = reader.stream_position()?;

            seek_table.push((
                times[0],
                StreamBatchDescriptor {
                    times,
                    data_offset: offset as u32,
                    compressed_len: header.data_compressed_len as u32,
                    decompressed_len: header.data_uncompressed_len as u32,
                },
            ));

            reader.seek_relative(header.data_compressed_len as i64)?;
        }

        seek_table.sort_by_key(|(k, _)| *k);

        reader.rewind()?;
        let file = reader.into_inner();

        let map = unsafe { Mmap::map(&file) }?;

        Ok(StreamDatabase {
            store: map,
            seek_table,
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
            db_manager,
            _spooky: PhantomData,
        })
    }

    #[inline(always)]
    fn decompressor(&self) -> VCRResult<Decompressor> {
        // let mut decompressor = if let Some(ref dict) = self.decoder {
        //     Decompressor::with_prepared_dictionary(dict)?
        // } else {
        //     Decompressor::new()?
        // };

        // decompressor.include_magicbytes(false)?;

        Ok(Decompressor::new()?)
    }

    #[inline(always)]
    fn get_data_range(
        &self,
        range: Range<usize>,
        decompressed_len: usize,
        decompressor: &mut Decompressor,
    ) -> VCRResult<Arc<Vec<u8>>> {
        let range = (range.start, range.end);
        if let Some(data) = self.cache.get(&range) {
            return Ok(data);
        }

        let data = &self.store[Range {
            start: range.0,
            end: range.1,
        }];
        let decompressed = Arc::new(decompressor.decompress(data, decompressed_len)?);
        self.cache.insert(range, Arc::clone(&decompressed));
        Ok(decompressed)
    }

    fn get_batch<'a>(
        &self,
        header: &'a StreamBatchDescriptor,
        decompressor: &mut Decompressor,
    ) -> VCRResult<StreamBatch<'a, I, I::Packed>> {
        let data = self.get_data_range(
            header.data_offset as usize
                ..header.data_offset as usize + header.compressed_len as usize,
            header.decompressed_len as usize,
            decompressor,
        )?;
        let data: Vec<I::Packed> = bitcode::decode(&data)?;
        Ok(StreamBatch {
            times: &header.times,
            data,
            _spooky: PhantomData,
        })
    }

    fn descriptor_by_time(&self, at: i64) -> Option<(usize, &StreamBatchDescriptor)> {
        let index = match self.seek_table.binary_search_by_key(&at, |(k, _)| *k) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        let (start_time, block) = &self.seek_table[index];

        if *start_time > at {
            None
        } else {
            Some((index, block))
        }
    }
}

impl<I: StreamComponent<Packed = P>, P: PackedStreamComponent<Unpacked = I>> EntityDatabase
    for StreamDatabase<I, P>
{
    type Record = StreamEntityWrapper<I>;

    fn from_single(path: impl AsRef<std::path::Path>) -> VCRResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn header_by_index(&self, index: u32) -> Option<&crate::vhs::DataHeader> {
        todo!()
    }

    fn index_from_id(&self, id: &[u8; 16]) -> Option<u32> {
        todo!()
    }

    fn get_entity_by_location(
        &self,
        location: &crate::EntityLocation,
    ) -> VCRResult<crate::OptionalEntity<Self::Record>> {
        todo!()
    }

    fn get_entities_by_location(
        &self,
        locations: &[crate::EntityLocation],
        force_single_thread: bool,
    ) -> VCRResult<Vec<crate::OptionalEntity<Self::Record>>> {
        todo!()
    }

    fn get_entity(
        &self,
        _id: &[u8; 16],
        at: i64,
    ) -> VCRResult<crate::OptionalEntity<Self::Record>> {
        let Some((_, block)) = self.descriptor_by_time(at) else {
            return Ok(None);
        };

        let data = self.get_batch(block, &mut self.decompressor()?)?;

        let Some(idx) = data.index_by_time(at) else {
            return Ok(None);
        };

        Ok(Some(data.unpack_one(idx, &self.db_manager)?))
    }

    fn get_first_entity(&self, id: &[u8; 16]) -> VCRResult<crate::OptionalEntity<Self::Record>> {
        todo!()
    }

    fn get_first_entities(
        &self,
        ids: &[[u8; 16]],
    ) -> VCRResult<Vec<crate::OptionalEntity<Self::Record>>> {
        todo!()
    }

    fn get_next_time(&self, id: &[u8; 16], at: i64) -> Option<i64> {
        todo!()
    }

    fn get_versions(
        &self,
        _id: &[u8; 16],
        before: i64,
        after: i64,
    ) -> VCRResult<Option<Vec<crate::ChroniclerEntity<Self::Record>>>> {
        let Some((start_index, _)) = self.descriptor_by_time(after) else {
            return Ok(None);
        };
        let Some((end_index, _)) = self.descriptor_by_time(before) else {
            return Ok(None);
        };

        let blocks = &self.seek_table[start_index..=end_index];
        let mut decompressor = self.decompressor()?;
        let mut versions = Vec::new();
        for (_, block) in blocks {
            let batch = self.get_batch(block, &mut decompressor)?;
            let start_index = batch.index_by_time(after).unwrap();
            let end_index = batch.index_by_time(before).unwrap();
            let mut batch_vers = batch
                .unpack_many(start_index, end_index, &self.db_manager)
                .collect::<VCRResult<Vec<_>>>()?;
            versions.append(&mut batch_vers);
        }

        Ok(Some(versions))
    }

    fn all_ids(&self) -> &[[u8; 16]] {
        &[[0u8; 16]]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}
