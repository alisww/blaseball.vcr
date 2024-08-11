use redb::TableDefinition;
use serde::Serialize;
use vcr_schemas::DynamicEntityType;
use xxhash_rust::xxh3::xxh3_128;

use super::DataHeader;
use crate::{timestamp_to_nanos, EntityLocation, RawChroniclerEntity, VCRResult};
use borsh::BorshSerialize;
use std::io::{self, Read, Write};
use std::marker::PhantomData;
use uuid::Uuid;
use vhs_diff::*;
use zstd::bulk::Compressor;

pub const HASH_TO_ENTITY_TABLE: TableDefinition<u128, EntityLocation> =
    TableDefinition::new("entity-locations");
pub type HeaderIndex = u32;


pub struct TapeEntity<T> {
    pub id: [u8; 16],
    pub times: Vec<i64>,
    pub data: Vec<T>,
}

impl<T> From<Vec<RawChroniclerEntity<T>>> for TapeEntity<T> {
    fn from(entity: Vec<RawChroniclerEntity<T>>) -> TapeEntity<T> {
        let id = *Uuid::parse_str(&entity[0].entity_id).unwrap().as_bytes();
        let times = entity
            .iter()
            .map(|v| timestamp_to_nanos(v.valid_from))
            .collect();
        let data = entity.into_iter().map(|v| v.data).collect();

        TapeEntity { id, times, data }
    }
}

impl<T: BorshSerialize> TapeEntity<T> {
    pub fn hashes(&self) -> Vec<u128> {
        let mut hashes = Vec::with_capacity(self.data.len());

        let mut serialize_buffer = Vec::new();
        for entity in &self.data {
            entity.serialize(&mut serialize_buffer).unwrap();
            hashes.push(xxh3_128(&serialize_buffer));
            serialize_buffer.clear();
        }

        hashes
    }
}

pub struct TapeRecorder<T: Serialize + Clone + Patch + Diff + Send + Sync, M: Write> {
    patch_out: M,
    compressor: Compressor<'static>,
    offset: u32,
    headers: Vec<DataHeader>,
    checkpoint_every: usize,
    _etype: DynamicEntityType,
    _record_type: PhantomData<T>,
}

impl<T: BorshSerialize + Serialize + Clone + Patch + Diff + Send + Sync, M: Write>
    TapeRecorder<T, M>
{
    pub fn new(
        etype: DynamicEntityType,
        patch_out: M,
        dict: Option<Vec<u8>>,
        compression_level: i32,
        checkpoint_every: usize,
    ) -> VCRResult<TapeRecorder<T, M>> {
        let mut compressor = if let Some(ref d) = dict {
            Compressor::with_dictionary(compression_level, d)?
        } else {
            Compressor::new(compression_level)?
        };

        compressor.include_dictid(false)?;
        compressor.include_magicbytes(false)?;

        Ok(TapeRecorder {
            _etype: etype,
            patch_out,
            compressor,
            offset: 0,
            headers: Vec::new(),
            checkpoint_every,
            _record_type: PhantomData,
        })
    }

    pub fn add_entity(
        &mut self,
        entity: TapeEntity<T>,
        hash_table: &mut redb::Table<'_, u128, EntityLocation>,
    ) -> VCRResult<()> {
        let hashes = entity.hashes();
        let header_index = self.headers.len() as u32;

        // let location = EntityLocation {
        //     entity_type: self.etype,
        //     header_index,
        // };

        for (idx, hash) in hashes.into_iter().enumerate() {
            hash_table.insert(hash, EntityLocation {
                header_index: header_index,
                time_index: idx.try_into().unwrap()
            })?;
        }

        let (bytes, checkpoint_positions) = encode_entity(entity.data, self.checkpoint_every)?;

        let compressed_patches = self.compressor.compress(&bytes[..])?;

        let header = DataHeader {
            id: entity.id,
            times: entity.times,
            compressed_len: compressed_patches.len() as u32,
            decompressed_len: bytes.len() as u32,
            offset: self.offset,
            checkpoint_every: self.checkpoint_every,
            checkpoint_positions,
        };

        self.offset += header.compressed_len;
        self.headers.push(header);

        self.patch_out.write_all(&compressed_patches)?;

        Ok(())
    }

    /// finish, compress header, return handles for header file + patch database.
    pub fn finish(self) -> VCRResult<(Vec<u8>, M)> {
        let header_bytes = rmp_serde::to_vec(
            &self
                .headers
                .into_iter()
                .map(|v| v.encode())
                .collect::<Vec<_>>(),
        )?;

        // header_out.set_pledged_src_size(Some(header_bytes.len() as u64))?;

        // header_out.write_all(&header_bytes)?;
        // header_out.flush()?;

        // let header_handle = header_out.finish()?;

        // self.patch_out.flush()?;

        Ok((header_bytes, self.patch_out))
    }
}

/// Merges header, dictionary and database into a single file.
pub fn merge_tape(
    header: Vec<u8>,
    mut db: impl Read,
    dict: Option<impl Read>,
    mut out: impl Write,
) -> VCRResult<()> {
    // let mut header_bytes = Vec::new();
    // header.read_to_end(&mut header_bytes)?;
    let dict_bytes = if let Some(mut dict_reader) = dict {
        let mut buf = Vec::new();
        dict_reader.read_to_end(&mut buf)?;
        buf
    } else {
        Vec::new()
    };

    out.write_all(&dict_bytes.len().to_le_bytes())?;
    out.write_all(&dict_bytes)?;

    let mut header_out = zstd::Encoder::new(Vec::with_capacity(header.len()), 21)?;
    header_out.set_pledged_src_size(Some(header.len() as u64))?;
    header_out.long_distance_matching(true)?;

    header_out.write_all(&header)?;

    let compressed_header_bytes = header_out.finish()?;

    out.write_all(&(compressed_header_bytes.len() as u64).to_le_bytes())?;
    out.write_all(&compressed_header_bytes)?;

    io::copy(&mut db, &mut out)?;

    Ok(())
}

pub struct DictTrainer {
    sample_sizes: Vec<usize>,
    samples: Vec<u8>,
    checkpoint_every: usize,
}

impl DictTrainer {
    pub fn new(checkpoint_every: usize) -> DictTrainer {
        DictTrainer {
            sample_sizes: Vec::new(),
            samples: Vec::new(),
            checkpoint_every,
        }
    }

    pub fn add_entity<T: Diff + Clone + Serialize>(&mut self, data: Vec<T>) -> VCRResult<()> {
        let (mut bytes, _) = encode_entity(data, self.checkpoint_every)?;

        self.sample_sizes.push(bytes.len());
        self.samples.append(&mut bytes);

        Ok(())
    }

    pub fn train(self, dict_size: usize) -> VCRResult<Vec<u8>> {
        Ok(zstd::dict::from_continuous(
            &self.samples,
            &self.sample_sizes,
            dict_size,
        )?)
    }
}

// returns (bytes, checkpoint_positions)
pub fn encode_entity<T: Diff + Serialize + Clone>(
    data: Vec<T>,
    checkpoint_every: usize,
) -> VCRResult<(Vec<u8>, Vec<usize>)> {
    let mut patches: Vec<OwnedPatch> = Vec::new();
    let mut bytes: Vec<u8> = Vec::new();
    let mut checkpoint_positions: Vec<usize> = Vec::new();

    if data.len() > 1 {
        for (i, vals) in data.windows(2).enumerate() {
            // if we're at a checkpoint, serialize the current patches, then write a full version of the entity
            if i % checkpoint_every == 0 {
                let mut cur_patches: Vec<u8> =
                    rmp_serde::to_vec(&patches.drain(..).collect::<Vec<OwnedPatch>>())?;
                bytes.append(&mut cur_patches);

                // write position of checkpoint
                checkpoint_positions.push(bytes.len());

                let mut serialized_cur = rmp_serde::to_vec(&vals[0])?;
                bytes.append(&mut serialized_cur);
            }

            patches.push(vals[0].diff(vals[1].clone()));
        }

        if !patches.is_empty() {
            let mut cur_patches: Vec<u8> =
                rmp_serde::to_vec(&patches.into_iter().collect::<Vec<OwnedPatch>>())?;
            bytes.append(&mut cur_patches);
        }

        // in this case, the last item will be missing since it won't be included in - so add a checkpoint
        if data.len() % 2 != 0 {
            // write position of checkpoint
            checkpoint_positions.push(bytes.len());

            let mut serialized_cur = rmp_serde::to_vec(&data.last().unwrap())?;
            bytes.append(&mut serialized_cur);
        }
    // if an entity only has one version, just serialize that version and no patches
    } else if !data.is_empty() {
        bytes = rmp_serde::to_vec(&data[0])?;
        // this does nothing besides add a nil length marker, i think
        let mut cur_patches: Vec<u8> =
            rmp_serde::to_vec(&patches.into_iter().collect::<Vec<OwnedPatch>>())?;
        bytes.append(&mut cur_patches);

        // the only checkpoint is at the very start!
        checkpoint_positions.push(0);
    }

    Ok((bytes, checkpoint_positions))
}
