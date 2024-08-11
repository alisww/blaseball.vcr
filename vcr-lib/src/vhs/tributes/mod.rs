use dusk_varint::VarInt;
use memmap2::Mmap;

use moka::sync::Cache;

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self},
    path::Path,
    sync::Arc,
    time::Duration,
};
use uuid::Uuid;
use vcr_schemas::{TributeValue, Tributes};

use xxhash_rust::xxh3;
use zstd::{bulk::Decompressor, dict::DecoderDictionary};

mod command;
pub mod recorder;

use command::TributeCommand;

use crate::{ChroniclerEntity, EntityDatabase, VCRResult};

use super::TapeComponents;

const TEAM_EPOCH: i64 = 1_623_600_000_000_000_000;

struct BlockReader<'a> {
    slice: &'a [u8],
}

impl<'a> BlockReader<'a> {
    #[inline(always)]
    fn has_remaining(&self) -> bool {
        !self.slice.is_empty()
    }

    #[inline(always)]
    fn command(&mut self) -> TributeCommand {
        let (lhs, rhs) = self.slice.split_at(4);
        let val = unsafe { (lhs.as_ptr() as *const u32).read_unaligned() };

        self.slice = rhs;

        TributeCommand::from_bits(val)
    }

    #[inline(always)]
    fn varint(&mut self) -> Option<i64> {
        i64::decode_var(self.slice).map(|(val, space)| {
            let (_, rhs) = self.slice.split_at(space);
            self.slice = rhs;
            val
        })
    }

    fn apply(
        mut self,
        players: &mut BTreeMap<u16, TributeValue>,
        teams: &mut BTreeMap<u16, TributeValue>,
    ) {
        use self::command::{CommandKind::*, TributeAffects::*};

        while self.has_remaining() {
            let command = self.command();
            let id = command.get(TributeCommand::ID);

            match command.get(TributeCommand::KIND) {
                Delta => {
                    let entry = match command.get(TributeCommand::AFFECTS) {
                        Team => teams.entry(id),
                        Player => players.entry(id),
                    };

                    let delta = if command.get(TributeCommand::EMBEDDED_DELTA) {
                        command.get(TributeCommand::DELTA) as i64
                    } else {
                        self.varint().unwrap()
                    };

                    let entry = entry.or_insert(TributeValue(0, false));
                    entry.0 += delta;
                    entry.1 = true;
                }
                Remove => match command.get(TributeCommand::AFFECTS) {
                    Team => {
                        if let Some(TributeValue(_, is_present)) = teams.get_mut(&id) {
                            *is_present = false
                        }
                    }
                    Player => {
                        if let Some(TributeValue(_, is_present)) = players.get_mut(&id) {
                            *is_present = false
                        }
                    }
                },
            }
        }
    }
}

pub struct TributesDatabase {
    times: Vec<i64>,
    store: CompressedTributeStore,
}

impl TributesDatabase {
    fn get_at(&self, at: i64) -> (i64, Tributes) {
        let index = match self.times.binary_search_by(|probe| probe.cmp(&at)) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };

        let at = self.times[index];

        let mut players = BTreeMap::new();
        let mut teams = BTreeMap::new();
        // let mut latest_time = 0i64;

        for idx in 0..=index {
            self.apply_block(idx, &mut players, &mut teams).unwrap();
        }

        (
            at,
            if at < TEAM_EPOCH {
                Tributes::Players(players)
            } else {
                Tributes::PlayersAndTeams { players, teams }
            },
        )
    }

    fn get_versions_inner(&self, before: i64, after: i64) -> Vec<(i64, Tributes)> {
        let index = match self.times.binary_search_by(|probe| probe.cmp(&before)) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };

        let mut versions: Vec<(i64, Tributes)> = Vec::with_capacity(index);

        let mut players = BTreeMap::new();
        let mut teams = BTreeMap::new();

        for (idx, time) in self.times[..=index].iter().enumerate() {
            self.apply_block(idx, &mut players, &mut teams).unwrap();
            if *time > after {
                versions.push((
                    *time,
                    if *time < TEAM_EPOCH {
                        Tributes::Players(players.clone())
                    } else {
                        Tributes::PlayersAndTeams {
                            players: players.clone(),
                            teams: teams.clone(),
                        }
                    },
                ));
            }
        }

        versions
    }

    #[inline(always)]
    fn apply_block(
        &self,
        idx: usize,
        players: &mut BTreeMap<u16, TributeValue>,
        teams: &mut BTreeMap<u16, TributeValue>,
    ) -> io::Result<()> {
        BlockReader {
            slice: &self.store.get_block(idx)?,
        }
        .apply(players, teams);
        Ok(())
    }
}

impl EntityDatabase for TributesDatabase {
    type Record = Tributes;

    fn from_single(path: impl AsRef<Path>) -> VCRResult<TributesDatabase> {
        let TapeComponents {
            dict,
            header,
            store,
        } = TapeComponents::<TributesTapeHeader>::split(path)?;

        let store = CompressedTributeStore {
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
            store,
            blocks: header.positions,
            dict: dict.unwrap(),
        };

        Ok(TributesDatabase {
            times: header.times,
            store,
        })
    }

    fn header_by_index(&self, _index: u32) -> Option<&super::DataHeader> {
        unimplemented!()
    }

    fn index_from_id(&self, _id: &[u8; 16]) -> Option<u32> {
        unimplemented!()
    }

    fn get_entities_by_location(
        &self,
        _locations: &[crate::EntityLocation],
        _force_single_thread: bool,
    ) -> VCRResult<Vec<crate::OptionalEntity<Self::Record>>> {
        todo!()
    }

    fn get_entity_by_location(
        &self,
        _location: &crate::EntityLocation,
    ) -> VCRResult<crate::OptionalEntity<Self::Record>> {
        todo!()
    }

    fn get_entity(
        &self,
        _: &[u8; 16],
        at: i64,
    ) -> crate::VCRResult<crate::OptionalEntity<Self::Record>> {
        let (time, data) = self.get_at(at);
        Ok(Some(crate::ChroniclerEntity {
            entity_id: Uuid::nil().into_bytes(),
            valid_from: time,
            data,
        }))
    }

    fn get_first_entity(
        &self,
        id: &[u8; 16],
    ) -> crate::VCRResult<crate::OptionalEntity<Self::Record>> {
        self.get_entity(id, 0)
    }

    fn get_first_entities(
        &self,
        _ids: &[[u8; 16]],
    ) -> crate::VCRResult<Vec<crate::OptionalEntity<Self::Record>>> {
        Ok(vec![self.get_entity(Uuid::nil().as_bytes(), 0)?])
    }

    fn get_next_time(&self, _: &[u8; 16], at: i64) -> Option<i64> {
        self.times
            .get(match self.times.binary_search_by(|probe| probe.cmp(&at)) {
                Ok(idx) => idx,
                Err(idx) => idx,
            })
            .copied()
    }

    fn get_versions(
        &self,
        _: &[u8; 16],
        before: i64,
        after: i64,
    ) -> crate::VCRResult<Option<Vec<crate::ChroniclerEntity<Self::Record>>>> {
        Ok(Some(
            self.get_versions_inner(before, after)
                .into_iter()
                .map(|(time, data)| ChroniclerEntity {
                    entity_id: Uuid::nil().into_bytes(),
                    valid_from: time,
                    data,
                })
                .collect(),
        ))
    }

    fn all_ids(&self) -> &[[u8; 16]] {
        &[[0u8; 16]]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct TributePosition {
    start: u64,
    end: u64,
    uncompressed_len: u64,
}

pub struct CompressedTributeStore {
    cache: Cache<usize, Arc<Vec<u8>>, xxh3::Xxh3Builder>,
    store: Mmap,
    blocks: Vec<TributePosition>, // start, end, uncompressed_len
    dict: DecoderDictionary<'static>,
}

impl CompressedTributeStore {
    pub fn get_block(&self, idx: usize) -> io::Result<Arc<Vec<u8>>> {
        self.cache
            .try_get_with(idx, || -> io::Result<Arc<Vec<u8>>> {
                let TributePosition {
                    start,
                    end,
                    uncompressed_len,
                } = self.blocks[idx];
                let mut decompressor = Decompressor::with_prepared_dictionary(&self.dict)?;
                let mut buf = vec![0u8; uncompressed_len as usize];

                decompressor
                    .decompress_to_buffer(&self.store[start as usize..end as usize], &mut buf)?;

                Ok(Arc::new(buf))
            })
            .map_err(|e| io::Error::new(e.kind(), e))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TributesTapeHeader {
    pub times: Vec<i64>,
    pub positions: Vec<TributePosition>,
}
