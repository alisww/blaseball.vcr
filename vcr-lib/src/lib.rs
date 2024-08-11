#![feature(buf_read_has_data_left)]

mod err;
pub mod site;
#[macro_use]
pub mod utils;
use std::path::Path;

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
pub use stream_data::db::StreamEntityWrapper;
pub use utils::*;
pub mod feed;
pub use err::*;
pub mod db_manager;
pub mod etypes;
pub mod vhs;

mod chron_types;
pub use chron_types::*;
use vhs::DataHeader;

pub mod stream_data;

// use chrono::{DateTime, Utc};
// use rocket::FromFormField;
// use serde::{Deserialize, Serialize};
// use serde_json::Value as JSONValue;

pub type VCRResult<T> = Result<T, VCRError>;
pub type OptionalEntity<T> = Option<ChroniclerEntity<T>>;
pub type RangeTuple = (usize, usize);

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Serialize, Deserialize, Encode, Decode)]
pub struct EntityLocation {
    pub header_index: u32,
    pub time_index: u32,
}

impl redb::Value for EntityLocation {
    type SelfType<'a>
     = EntityLocation where Self: 'a;

    type AsBytes<'a>
     = [u8; 8] where Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(8)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        EntityLocation {
            header_index: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            time_index: u32::from_le_bytes(data[4..].try_into().unwrap()),
        }
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        let mut out = [0u8; 8];
        out[0..4].copy_from_slice(&value.header_index.to_le_bytes());
        out[4..].copy_from_slice(&value.time_index.to_le_bytes());

        out
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("blaseball-vcr-entity-location")
    }
}

pub trait EntityDatabase {
    type Record;

    fn from_single(path: impl AsRef<Path>) -> VCRResult<Self>
    where
        Self: Sized;

    fn header_by_index(&self, index: u32) -> Option<&DataHeader>;

    fn index_from_id(&self, id: &[u8; 16]) -> Option<u32>;

    fn get_entity_by_location(
        &self,
        location: &EntityLocation,
    ) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_entities_by_location(
        &self,
        locations: &[EntityLocation],
        force_single_thread: bool,
    ) -> VCRResult<Vec<OptionalEntity<Self::Record>>>;

    fn get_entity(&self, id: &[u8; 16], at: i64) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_first_entity(&self, id: &[u8; 16]) -> VCRResult<OptionalEntity<Self::Record>>;

    fn get_first_entities(&self, ids: &[[u8; 16]]) -> VCRResult<Vec<OptionalEntity<Self::Record>>>;

    fn get_entities(
        &self,
        ids: &[[u8; 16]],
        at: i64,
    ) -> VCRResult<Vec<OptionalEntity<Self::Record>>> {
        ids.iter()
            .map(|id| self.get_entity(id, at))
            .collect::<VCRResult<Vec<OptionalEntity<Self::Record>>>>()
    }

    fn get_next_time(&self, id: &[u8; 16], at: i64) -> Option<i64>;

    fn get_versions(
        &self,
        id: &[u8; 16],
        before: i64,
        after: i64,
    ) -> VCRResult<Option<Vec<ChroniclerEntity<Self::Record>>>>;

    fn all_ids(&self) -> &[[u8; 16]];

    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct GameDate {
    pub day: i16,
    pub season: i8,
    pub tournament: i8,
}

impl GameDate {
    pub const fn to_bytes(&self) -> [u8; 4] {
        let [day_a, day_b] = self.day.to_le_bytes();
        [
            day_a,
            day_b,
            self.season.to_le_bytes()[0],
            self.tournament.to_le_bytes()[0],
        ]
    }

    pub const fn from_bytes([day_a, day_b, season, tournament]: [u8; 4]) -> GameDate {
        GameDate {
            day: i16::from_le_bytes([day_a, day_b]),
            season: i8::from_le_bytes([season]),
            tournament: i8::from_le_bytes([tournament]),
        }
    }
}

// hack so we can use call_method_by_type for Database::from_single
pub mod db_wrapper {
    use crate::vhs::db::Database;
    use crate::VCRResult;
    use crate::{db_manager::*, EntityDatabase};

    pub fn from_single_and_insert<
        T: Clone
            + vhs_diff::Patch
            + vhs_diff::Diff
            + serde::de::DeserializeOwned
            + Send
            + Sync
            + serde::Serialize
            + 'static,
    >(
        manager: &mut DatabaseManager,
        path: &std::path::Path,
    ) -> VCRResult<()> {
        let v: Database<T> = Database::from_single(path)?;
        manager.insert(v);
        Ok(())
    }
}

pub struct SliceReader<'a> {
    bytes: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn read_array<const N: usize>(&mut self) -> &'a [u8; N] {
        let (lhs, rhs) = self.bytes.split_at(N);
        self.bytes = rhs;

        unsafe { &*(lhs.as_ptr() as *const [u8; N]) }
    }

    pub fn read_slice(&mut self, len: usize) -> &'a [u8] {
        let (lhs, rhs) = self.bytes.split_at(len);
        self.bytes = rhs;
        lhs
    }

    pub fn read_str(&mut self) -> &'a str {
        let len = u16::from_le_bytes(*self.read_array::<2>());
        unsafe { std::str::from_utf8_unchecked(self.read_slice(len as usize)) }
    }

    pub fn read_varlen_slice(&mut self) -> &'a [u8] {
        let len = u16::from_le_bytes(*self.read_array::<2>());
        self.read_slice(len as usize)
    }
}

pub fn write_str(st: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(st.len() as u16).to_le_bytes());
    out.extend_from_slice(st.as_bytes());
}

pub fn write_slice(st: &[u8], out: &mut Vec<u8>) {
    out.extend_from_slice(&(st.len() as u16).to_le_bytes());
    out.extend_from_slice(st);
}
