use crate::etypes::DynamicEntity;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::timestamp_from_nanos;

pub struct ChroniclerEntity<T> {
    pub entity_id: [u8; 16],
    pub valid_from: i64,
    pub data: T,
}

impl<T> ChroniclerEntity<T> {
    pub fn as_game_update(self) -> GameUpdateWrapper<T> {
        GameUpdateWrapper { inner: self }
    }
}

impl<T: Into<DynamicEntity>> ChroniclerEntity<T> {
    #[inline(always)]
    pub fn erase(self) -> ChroniclerEntity<DynamicEntity> {
        ChroniclerEntity {
            entity_id: self.entity_id,
            valid_from: self.valid_from,
            data: self.data.into(),
        }
    }
}

impl<T: Serialize> Serialize for ChroniclerEntity<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("ChroniclerEntity", 5)?;
        ser.serialize_field("entityId", &Uuid::from_bytes(self.entity_id))?;
        ser.serialize_field(
            "validFrom",
            &(iso8601_timestamp::Timestamp::UNIX_EPOCH
                + iso8601_timestamp::Duration::nanoseconds(self.valid_from)),
        )?;
        // we don't store these
        ser.serialize_field("validTo", &())?;
        ser.serialize_field("hash", "")?; // there's probably a way to add hashing here behind a compile feature - i'm not sure it's worth it, tho
                                          // -
        ser.serialize_field("data", &self.data)?;
        ser.end()
    }
}

// wrapper to serialize game updates in the way chron v1 does
#[repr(transparent)]
pub struct GameUpdateWrapper<T> {
    inner: ChroniclerEntity<T>,
}

impl<T: Serialize> Serialize for GameUpdateWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("GameUpdateWrapper", 4)?;
        ser.serialize_field("gameId", &Uuid::from_bytes(self.inner.entity_id))?;
        ser.serialize_field("timestamp", &timestamp_from_nanos(self.inner.valid_from))?;
        ser.serialize_field("hash", "")?; // there's probably a way to add hashing here behind a compile feature - i'm not sure it's worth it, tho
                                          // -
        ser.serialize_field("data", &self.inner.data)?;
        ser.end()
    }
}

// as returned from the actual chron api
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawChroniclerEntity<T> {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: iso8601_timestamp::Timestamp,
    pub valid_to: Option<String>,
    pub data: T,
}
