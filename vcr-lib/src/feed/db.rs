use super::block::*;
use super::header::PackedHeader;
use super::index::{EventIdChunk, FeedIndexCollection};
use super::BlockHeader;
use crate::vhs::TapeComponents;
use crate::VCRResult;
use memmap2::Mmap;
use moka::sync::Cache;
use serde::ser::{Error, Serialize, SerializeSeq, Serializer};
use std::cell::Cell;
use vcr_lookups::UuidShell;

use std::collections::btree_map;
use std::fs::File;
use std::ops::RangeBounds;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use xxhash_rust::xxh3;
use zstd::bulk::Decompressor;
use zstd::dict::DecoderDictionary;

pub struct FeedDatabase {
    inner: Mmap,
    dict: Option<DecoderDictionary<'static>>,
    headers: Vec<BlockHeader>,
    cache: Cache<u16, Arc<EventBlock>, xxh3::Xxh3Builder>,
    pub indexes: FeedIndexCollection,
}

impl FeedDatabase {
    pub fn from_tape(
        path: impl AsRef<Path>,
        indexes_path: Option<impl AsRef<Path>>,
    ) -> VCRResult<FeedDatabase> {
        let TapeComponents {
            dict,
            header,
            store,
        } = TapeComponents::<PackedHeader>::split(path)?;

        let raw_headers = header.decode();

        let headers: Vec<BlockHeader> = {
            let mut offset = 0;
            let mut headers: Vec<BlockHeader> = Vec::with_capacity(raw_headers.len());

            for header in raw_headers {
                let compressed_len = header.compressed_len;

                headers.push(BlockHeader {
                    compressed_len,
                    decompressed_len: header.decompressed_len,
                    start_time: header.start_time,
                    event_positions: header.event_positions,
                    metadata: header.metadata,
                    offset,
                });

                offset += compressed_len;
            }

            headers
        };

        let indexes: FeedIndexCollection = if let Some(path) = indexes_path {
            rmp_serde::from_read(zstd::Decoder::new(File::open(path)?)?)?
        } else {
            FeedIndexCollection::default()
        };

        Ok(FeedDatabase {
            headers,
            indexes,
            dict,
            inner: store,
            cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(20 * 60))
                .time_to_idle(Duration::from_secs(10 * 60))
                .build_with_hasher(xxh3::Xxh3Builder::new()),
        })
    }

    #[inline(always)]
    fn decompressor(&self) -> VCRResult<Decompressor> {
        let mut decompressor = if let Some(ref dict) = self.dict {
            Decompressor::with_prepared_dictionary(dict)?
        } else {
            Decompressor::new()?
        };

        decompressor.include_magicbytes(false)?;

        Ok(decompressor)
    }

    pub fn get_block_by_index(&self, index: u16) -> VCRResult<Arc<EventBlock>> {
        if let Some(cached) = self.cache.get(&index) {
            return Ok(cached);
        }

        let header = &self.headers[index as usize];

        let data =
            &self.inner[header.offset as usize..(header.offset + header.compressed_len) as usize];
        let decompressed = self
            .decompressor()?
            .decompress(data, header.decompressed_len as usize)?;

        let block = Arc::new(EventBlock {
            bytes: decompressed,
            event_positions: header.event_positions.clone(),
            meta: header.metadata,
        });

        self.cache.insert(index, Arc::clone(&block));

        Ok(block)
    }

    pub fn get_block_by_time(&self, at: i64) -> VCRResult<Arc<EventBlock>> {
        let index = match self.headers.binary_search_by_key(&at, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        };

        self.get_block_by_index(index as u16)
    }

    pub fn get_events_after(&self, after: i64, count: usize) -> EventRangeSerializer<'_> {
        let block_index = match self.headers.binary_search_by_key(&after, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        } as u16;

        EventRangeSerializer {
            inner: Cell::new(Some(EventRangeIter {
                db: self,
                block_index,
                go_up: true,
                count_left: count,
            })),
            before: i64::MAX,
            after,
        }
    }

    pub fn get_events_before(&self, before: i64, count: usize) -> EventRangeSerializer<'_> {
        let block_index = match self.headers.binary_search_by_key(&before, |v| v.start_time) {
            Ok(i) => i,
            Err(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
        } as u16;

        EventRangeSerializer {
            inner: Cell::new(Some(EventRangeIter {
                db: self,
                block_index,
                go_up: false,
                count_left: count,
            })),
            before,
            after: 0,
        }
    }

    pub fn events_by_game<R: RangeBounds<i64> + Copy>(
        &self,
        game: impl Into<UuidShell>,
        range: R,
        limit: usize,
        offset: usize,
    ) -> Option<EventIndexResultsSerializer<'_, '_, R>> {
        Some(self.events_from_index_results(
            self.indexes.by_game(game, range)?,
            range,
            limit,
            offset,
        ))
    }

    pub fn events_by_team<R: RangeBounds<i64> + Copy>(
        &self,
        team: impl Into<UuidShell>,
        range: R,
        limit: usize,
        offset: usize,
    ) -> Option<EventIndexResultsSerializer<'_, '_, R>> {
        Some(self.events_from_index_results(
            self.indexes.by_team(team, range)?,
            range,
            limit,
            offset,
        ))
    }

    pub fn events_by_player<R: RangeBounds<i64> + Copy>(
        &self,
        player: impl Into<UuidShell>,
        range: R,
        limit: usize,
        offset: usize,
    ) -> Option<EventIndexResultsSerializer<'_, '_, R>> {
        Some(self.events_from_index_results(
            self.indexes.by_player(player, range)?,
            range,
            limit,
            offset,
        ))
    }

    fn events_from_index_results<'a, 'b, R: RangeBounds<i64>>(
        &'b self,
        iter: btree_map::Range<'a, i64, EventIdChunk>,
        range: R,
        limit: usize,
        offset: usize,
    ) -> EventIndexResultsSerializer<'a, 'b, R> {
        EventIndexResultsSerializer {
            inner: Cell::new(Some(EventIndexResultsState {
                iter,
                db: self,
                range,
                limit,
                offset,
            })),
        }
    }
}

pub struct EventRangeSerializer<'a> {
    inner: Cell<Option<EventRangeIter<'a>>>,
    before: i64,
    after: i64,
}

impl<'a> Serialize for EventRangeSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut iter = self.inner.take().ok_or(S::Error::custom(
            "NEventsAfterSerializer: inner iterator already consumed",
        ))?;
        let mut seq = serializer.serialize_seq(Some(iter.count_left))?;

        while let Some(block) = iter.next() {
            let block = block.map_err(S::Error::custom)?;
            let mut events = block.all_events();

            if !iter.go_up {
                events.retain(|v| v.created < self.before);
            } else {
                events.retain(|v| v.created > self.after);
            }

            if !iter.go_up {
                events.reverse();
            }

            events.truncate(iter.count_left);

            iter.count_left -= events.len();

            for event in events {
                seq.serialize_element(&event)?;
            }
        }

        seq.end()
    }
}

pub struct EventRangeIter<'a> {
    db: &'a FeedDatabase,
    block_index: u16,
    go_up: bool, // if true, increment block_index every step; else, subtract
    count_left: usize,
}

impl<'a> Iterator for EventRangeIter<'a> {
    type Item = VCRResult<Arc<EventBlock>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.go_up && self.block_index as usize >= self.db.headers.len() {
            return None;
        }

        let block = match self.db.get_block_by_index(self.block_index) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        if self.go_up {
            self.block_index = self.block_index.checked_add(1)?;
        } else {
            self.block_index = self.block_index.checked_sub(1)?;
        }

        Some(Ok(block))
    }
}

pub struct EventIndexResultsSerializer<'a, 'b, R: RangeBounds<i64>> {
    inner: Cell<Option<EventIndexResultsState<'a, 'b, R>>>,
}

struct EventIndexResultsState<'a, 'b, R: RangeBounds<i64>> {
    iter: btree_map::Range<'a, i64, EventIdChunk>,
    db: &'b FeedDatabase,
    range: R,
    limit: usize,
    offset: usize,
}

impl<'a, 'b, R: RangeBounds<i64>> Serialize for EventIndexResultsSerializer<'a, 'b, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let EventIndexResultsState {
            iter,
            db,
            range,
            limit,
            offset,
        } = self.inner.take().ok_or(S::Error::custom(
            "NEventsAfterSerializer: inner iterator already consumed",
        ))?;
        let mut seq = serializer.serialize_seq(None)?;

        let mut events_total = 0;

        'outer: for (_, chunk) in iter {
            let block = db
                .get_block_by_index(chunk.chunk)
                .map_err(S::Error::custom)?;

            for id in &chunk.ids {
                let event = block.event_at_index(*id as usize).unwrap();
                if range.contains(&event.created) {
                    events_total += 1;

                    if events_total >= offset {
                        seq.serialize_element(&event)?;
                    }

                    if events_total >= limit {
                        break 'outer;
                    }
                }
            }
        }

        seq.end()
    }
}
