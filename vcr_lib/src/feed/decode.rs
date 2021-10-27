use super::*;
use crate::{VCRError, VCRResult};
use chrono::{DateTime, TimeZone, Utc};
use memmap2::{Mmap, MmapOptions};
use moka::sync::Cache;
use rayon::prelude::*;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use uuid::Uuid;
use zstd::dict::DecoderDictionary;

fn make_offset_table<R: Read>(mut reader: R) -> Vec<(DateTime<Utc>, (u32, u16))> {
    let mut last_position: u64 = 0;
    let mut index: Vec<(DateTime<Utc>, (u32, u16))> = Vec::with_capacity(5110062);

    loop {
        let mut snowflake: Vec<u8> = vec![0; 6];
        if reader.read_exact(&mut snowflake).is_err() {
            break;
        }

        let position_delta = u16::from_be_bytes(snowflake[0..2].try_into().unwrap());
        let start_pos = last_position + position_delta as u64;

        if !index.is_empty() {
            let idx = index.len() - 1;
            let mut a = index[idx];
            a.1 .1 = (start_pos as u32 - a.1 .0) as u16;
            index[idx] = a;
        }

        index.push((
            Utc.timestamp(
                u32::from_be_bytes(snowflake[2..6].try_into().unwrap()) as i64,
                0,
            ),
            (start_pos as u32, 0u16),
        ));

        last_position = start_pos;
    }

    index.sort_unstable_by_key(|(t, _)| t.timestamp());

    index
}

pub struct FeedDatabase {
    offset_table: Vec<(DateTime<Utc>, (u32, u16))>,
    meta_index: MetaIndex,
    event_index: EventIndex,
    reader: Mmap,
    dictionary: DecoderDictionary<'static>,
    cache: Cache<u32, FeedEvent>,
}

impl FeedDatabase {
    pub fn from_files<P: AsRef<Path> + std::fmt::Debug>(
        position_index_path: P,
        db_file_path: P,
        dict_file_path: P,
        id_table_path: P,
        idx_file_path: P,
        cache_size: usize,
    ) -> VCRResult<FeedDatabase> {
        let id_file = File::open(id_table_path)?;
        let meta_idx: MetaIndex = rmp_serde::from_read(id_file)?;

        let idx_file = File::open(idx_file_path)?;
        let idx_r = BufReader::new(idx_file);
        let mut idx_decoder = zstd::Decoder::new(idx_r)?;

        let game_index = {
            let mut idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u16!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let player_index = {
            let mut idx: HashMap<u16, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u16!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let team_index = {
            let mut idx: HashMap<u8, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_u8!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let etype_index = {
            let mut idx: HashMap<i16, Vec<(u32, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let key = read_i16!(cursor);
                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_u32!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let phase_index = {
            let mut idx: HashMap<(u8, u8), Vec<(i64, (u32, u16))>> = HashMap::new();
            let idx_len = read_u32!(idx_decoder);
            let mut bytes: Vec<u8> = vec![0; idx_len as usize];
            idx_decoder.read_exact(&mut bytes)?;
            let mut cursor = Cursor::new(bytes);

            while cursor.position() < idx_len as u64 {
                let season_phase: u8 = read_u8!(cursor);
                let key = ((season_phase & 0xF) + 10, (season_phase >> 4) & 0xF);

                let klen: u64 = read_u32!(cursor) as u64;
                let start_pos = cursor.position();

                let entry = idx
                    .entry(key)
                    .or_insert_with(|| Vec::with_capacity(klen as usize));

                while (cursor.position() - start_pos) < klen {
                    entry.push((
                        read_i64!(cursor),
                        (read_u32!(cursor), decode_varint!(cursor)),
                    ));
                }
            }

            idx
        };

        let event_idx: EventIndex = EventIndex {
            player_index,
            team_index,
            phase_index,
            etype_index,
            game_index,
        };

        let position_index_file = File::open(position_index_path)?;
        let position_index_reader = BufReader::new(position_index_file);
        let position_index_decompressor = zstd::stream::Decoder::new(position_index_reader)?;
        let offset_table = make_offset_table(position_index_decompressor);

        let mut dictionary_file = File::open(dict_file_path)?;
        let mut dictionary: Vec<u8> = Vec::new();
        dictionary_file.read_to_end(&mut dictionary)?;

        let main_file = File::open(db_file_path)?;
        let main_file_reader = unsafe { MmapOptions::new().map(&main_file)? };

        Ok(FeedDatabase {
            offset_table,
            reader: main_file_reader,
            event_index: event_idx,
            meta_index: meta_idx,
            dictionary: DecoderDictionary::copy(&dictionary),
            cache: Cache::new(cache_size),
        })
    }

    pub fn read_event(
        &self,
        offset: u32,
        len: u16,
        timestamp: DateTime<Utc>,
    ) -> VCRResult<FeedEvent> {
        if let Some(ev) = self.cache.get(&offset) {
            return Ok(ev);
        }

        // let timestamp_raw = u32::from_be_bytes(snowflake[2..6].try_into().unwrap());
        // let timestamp = match self.millis_epoch_table.get(&(season, phase)) {
        //     Some(epoch) => (*epoch as i64) * 1000 + (timestamp_raw as i64),
        //     None => (timestamp_raw as i64) * 1000,
        // };

        let mut decoder = zstd::stream::Decoder::with_prepared_dictionary(
            if len == 0 {
                &self.reader[offset as usize..]
            } else {
                &self.reader[offset as usize..(offset + (len as u32)) as usize]
            },
            &self.dictionary,
        )?;

        let category: i8 = read_i8!(decoder);
        let etype: i16 = read_i16!(decoder);
        let day: i16 = {
            let d = read_u8!(decoder);
            if d == 255 {
                1522
            } else {
                d.into()
            }
        };

        let season_phase: u8 = read_u8!(decoder);
        let season: u8 = (season_phase & 0xF) + 10;
        let phase: u8 = (season_phase >> 4) & 0xF;

        let id = if phase == 13 {
            let mut uuid: [u8; 16] = [0; 16];
            decoder.read_exact(&mut uuid)?;
            Uuid::from_bytes(uuid)
        } else {
            Uuid::nil()
        };

        use EventDescription::*;
        let description = match EventDescription::from_type(etype) {
            Constant(s) => s.to_owned(),
            ConstantVariant(possibilities) => {
                let mut variant_byte: [u8; 1] = [0; 1];
                decoder.read_exact(&mut variant_byte)?;
                possibilities[u8::from_be(variant_byte[0]) as usize].to_owned()
            }
            ConstantMiddle(mid) => {
                let start_len = decode_varint!(decoder);
                let end_len = decode_varint!(decoder);

                let mut start_bytes: Vec<u8> = vec![0; start_len as usize];
                decoder.read_exact(&mut start_bytes)?;

                let mut end_bytes: Vec<u8> = vec![0; end_len as usize];
                decoder.read_exact(&mut end_bytes)?;

                String::from_utf8(start_bytes).unwrap()
                    + mid
                    + &String::from_utf8(end_bytes).unwrap()
            }
            VariableMiddle(possible_mid) => {
                let const_idx = read_u8!(decoder);
                let start_len = decode_varint!(decoder);

                if const_idx > possible_mid.len() as u8 {
                    let mut description_bytes: Vec<u8> = vec![0; start_len as usize];
                    decoder.read_exact(&mut description_bytes)?;
                    String::from_utf8(description_bytes).unwrap()
                } else {
                    let end_len = decode_varint!(decoder);

                    let mut start_bytes: Vec<u8> = vec![0; start_len as usize];
                    decoder.read_exact(&mut start_bytes)?;

                    let mut end_bytes: Vec<u8> = vec![0; end_len as usize];
                    decoder.read_exact(&mut end_bytes)?;

                    String::from_utf8(start_bytes).unwrap()
                        + possible_mid[const_idx as usize]
                        + &String::from_utf8(end_bytes).unwrap()
                }
            }
            VariablePrefix(possible_pfx) => {
                let const_idx = read_u8!(decoder);
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                if const_idx > possible_pfx.len() as u8 {
                    String::from_utf8(description_bytes).unwrap()
                } else {
                    possible_pfx[const_idx as usize].to_owned()
                        + &String::from_utf8(description_bytes).unwrap()
                }
            }
            VariableSuffix(possible_sfx) => {
                let const_idx = read_u8!(decoder);
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                if const_idx > possible_sfx.len() as u8 {
                    String::from_utf8(description_bytes).unwrap()
                } else {
                    String::from_utf8(description_bytes).unwrap() + possible_sfx[const_idx as usize]
                }
            }
            Suffix(sfx) => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                String::from_utf8(description_bytes).unwrap() + sfx
            }
            Prefix(pfx) => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;

                pfx.to_owned() + &String::from_utf8(description_bytes).unwrap()
            }
            Variable => {
                let description_len = decode_varint!(decoder);
                let mut description_bytes: Vec<u8> = vec![0; description_len as usize];
                decoder.read_exact(&mut description_bytes)?;
                String::from_utf8(description_bytes).unwrap()
            }
        };

        let player_tag_len = read_u8!(decoder);
        let mut player_tag_bytes: Vec<u8> = vec![0; (player_tag_len * 2) as usize];
        decoder.read_exact(&mut player_tag_bytes)?;

        let team_tag_len = read_u8!(decoder);
        let mut team_tag_bytes: Vec<u8> = vec![0; team_tag_len as usize];
        decoder.read_exact(&mut team_tag_bytes)?;

        let game_tag_len = read_u8!(decoder);
        let mut game_tag_bytes: Vec<u8> = vec![0; (game_tag_len * 2) as usize];
        decoder.read_exact(&mut game_tag_bytes)?;

        let mut metadata_bytes: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut metadata_bytes)?;

        let player_tags: Vec<Uuid> = {
            let mut player_tag_ids: Vec<u16> = Vec::new();
            while !player_tag_bytes.is_empty() {
                player_tag_ids.push(u16::from_be_bytes([
                    player_tag_bytes.remove(0),
                    player_tag_bytes.remove(0),
                ]));
            }

            player_tag_ids
                .into_iter()
                .map(|id| self.meta_index.player_tags[&id])
                .collect()
        };

        let team_tags: Vec<Uuid> = {
            let mut team_tag_ids: Vec<u8> = Vec::new();
            while !team_tag_bytes.is_empty() {
                team_tag_ids.push(u8::from_be_bytes([team_tag_bytes.remove(0)]));
            }

            team_tag_ids
                .into_iter()
                .map(|id| self.meta_index.team_tags[&id])
                .collect()
        };

        let game_tags: Vec<Uuid> = {
            let mut game_tag_ids: Vec<u16> = Vec::new();
            while !game_tag_bytes.is_empty() {
                game_tag_ids.push(u16::from_be_bytes([
                    game_tag_bytes.remove(0),
                    game_tag_bytes.remove(0),
                ]));
            }

            game_tag_ids
                .into_iter()
                .map(|id| self.meta_index.game_tags[&id])
                .collect()
        };

        let metadata: JSONValue = metadata::decode_metadata(etype, &metadata_bytes[..])?;

        let ev = FeedEvent {
            id,
            category,
            created: timestamp,
            day,
            season,
            nuts: 0,
            phase,
            player_tags: Some(player_tags),
            team_tags: Some(team_tags),
            game_tags: Some(game_tags),
            etype,
            tournament: -1,
            description,
            metadata,
        };

        self.cache.insert(offset, ev.clone());
        Ok(ev)
    }

    pub fn events_after(
        &self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut idx = match self
            .offset_table
            .binary_search_by_key(&timestamp.timestamp(), |(s, _)| s.timestamp())
        {
            Ok(i) => i,
            Err(i) => i,
        };

        let mut events: Vec<FeedEvent> = Vec::with_capacity(count);
        while idx < self.offset_table.len() && events.len() < count {
            let (time, (offset, length)) = self.offset_table[idx];
            let e = self.read_event(offset, length, time)?;
            if category == -3 || e.category == category {
                events.push(e);
            }

            idx += 1;
        }

        Ok(events)
    }

    pub fn events_before(
        &self,
        timestamp: DateTime<Utc>,
        count: usize,
        category: i8,
    ) -> VCRResult<Vec<FeedEvent>> {
        let mut idx = match self
            .offset_table
            .binary_search_by_key(&timestamp.timestamp(), |(s, _)| s.timestamp())
        {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        let mut events: Vec<FeedEvent> = Vec::with_capacity(count);
        while idx > 0 && events.len() < count {
            let (time, (offset, length)) = self.offset_table[idx];
            let e = self.read_event(offset, length, time)?;
            if category == -3 || e.category == category {
                events.push(e);
            }

            idx -= 1;
        }

        Ok(events)
    }

    pub fn events_by_phase(
        &self,
        season: u8,
        phase: u8,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        let ids = self.event_index.phase_index[&(season, phase)]
            .iter()
            .take(count)
            .copied()
            .collect::<Vec<(i64, (u32, u16))>>();

        ids.par_iter()
            .map(|(time, (offset, len))| {
                self.read_event(*offset, *len, Utc.timestamp_millis(*time))
            })
            .collect::<VCRResult<Vec<FeedEvent>>>()
    }

    pub fn events_by_type_and_time(
        &self,
        timestamp: DateTime<Utc>,
        etype: i16,
        count: usize,
    ) -> VCRResult<Vec<FeedEvent>> {
        if !self.event_index.etype_index.contains_key(&etype) {
            return Err(VCRError::IndexMissing);
        }

        let len = self.event_index.etype_index[&etype].len();

        let mut idx = 0;

        let mut events = Vec::with_capacity(count);
        while idx < len && events.len() < count {
            // AaaaaaaaaaaaaAAAAAAAAAAAAAAaaaAAAAAAAAAAAAaaAAAAAA
            let (time, (offset, length)) = self.event_index.etype_index[&etype][idx];

            let time = Utc.timestamp(time as i64, 0);
            if time <= timestamp {
                let e = self.read_event(offset, length, time)?;
                events.push(e);
            }

            idx += 1;
        }

        events.sort_by_key(|e| e.created.timestamp());
        events.dedup();
        events.reverse();

        Ok(events)
    }

    pub fn events_by_tag_and_time(
        &self,
        timestamp: DateTime<Utc>,
        tag: &Uuid,
        tag_type: TagType,
        count: usize,
        category: i8,
        etype: i16,
    ) -> VCRResult<Vec<FeedEvent>> {
        let tag: u16 = match tag_type {
            TagType::Game => *self
                .meta_index
                .reverse_game_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Team => *self
                .meta_index
                .reverse_team_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
            TagType::Player => *self
                .meta_index
                .reverse_player_tags
                .get(tag)
                .ok_or(VCRError::EntityNotFound)? as u16,
        };

        if etype != -1 && !self.event_index.etype_index.contains_key(&etype) {
            return Err(VCRError::IndexMissing);
        }

        let len = match tag_type {
            TagType::Game => self.event_index.game_index[&tag].len(),
            TagType::Team => self.event_index.team_index[&(tag as u8)].len(),
            TagType::Player => self.event_index.player_index[&tag].len(),
        };

        let mut idx = 0;

        let mut events = Vec::with_capacity(count);
        while idx < len && events.len() < count {
            // AaaaaaaaaaaaaAAAAAAAAAAAAAAaaaAAAAAAAAAAAAaaAAAAAA
            let (time, (offset, length)) = match tag_type {
                TagType::Game => self.event_index.game_index[&tag][idx],
                TagType::Team => self.event_index.team_index[&(tag as u8)][idx],
                TagType::Player => self.event_index.player_index[&tag][idx],
            };

            if etype == -1
                || self.event_index.etype_index[&etype].contains(&(time, (offset, length)))
            {
                let time = Utc.timestamp(time as i64, 0);
                if time <= timestamp {
                    let e = self.read_event(offset, length, time)?;
                    if category == -3 || e.category == category {
                        events.push(e);
                    }
                }
            }

            idx += 1;
        }

        events.sort_by_key(|e| e.created.timestamp());
        events.dedup();
        events.reverse();

        Ok(events)
    }
}
