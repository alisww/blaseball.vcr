use borsh::BorshSerialize;
use serde::{de::IntoDeserializer, Deserialize, Serialize, Serializer};
use std::hash::Hash;
use uuid::Uuid;

use crate::lookups::{GAME_ID_TABLE, PLAYER_ID_TABLE, TEAM_ID_TABLE};

#[derive(Clone, Debug, Copy, PartialOrd, Ord)]
pub enum UuidShell {
    RawUuid(Uuid),
    Tagged(UuidTag),
}

impl BorshSerialize for UuidShell {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        <Uuid as BorshSerialize>::serialize(&self.as_uuid(), writer)
    }
}

impl PartialEq for UuidShell {
    fn eq(&self, other: &Self) -> bool {
        self.as_uuid() == other.as_uuid()
    }
}

impl Eq for UuidShell {}

impl Hash for UuidShell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_uuid().hash(state)
    }
}

impl UuidShell {
    pub fn as_tag_value(self) -> Option<u16> {
        self.as_tag().and_then(|v| v.raw_tag())
    }

    pub fn as_tag(self) -> Option<UuidTag> {
        match self {
            UuidShell::RawUuid(_) => None,
            UuidShell::Tagged(v) => Some(v),
        }
    }

    pub fn find_tag(self) -> UuidShell {
        match self {
            Self::RawUuid(id) => {
                let res = if let Some(tag) = PLAYER_ID_TABLE.map(&id) {
                    UuidShell::Tagged(UuidTag::Player(*tag))
                } else if let Some(tag) = GAME_ID_TABLE.map(&id) {
                    UuidShell::Tagged(UuidTag::Game(*tag))
                } else if let Some(tag) = TEAM_ID_TABLE.map(&id) {
                    UuidShell::Tagged(UuidTag::Team(*tag))
                } else {
                    UuidShell::RawUuid(id)
                };

                assert_eq!(id, res.as_uuid());
                res
            }
            Self::Tagged(_) => self,
        }
    }

    pub fn as_uuid(&self) -> Uuid {
        match self {
            UuidShell::RawUuid(id) => *id,
            UuidShell::Tagged(v) => v.as_uuid(),
        }
    }
}

impl From<Uuid> for UuidShell {
    fn from(value: Uuid) -> Self {
        UuidShell::RawUuid(value).find_tag()
    }
}

impl Serialize for UuidShell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // if it's human readable, we serialize it as a formatted string
        if serializer.is_human_readable() {
            let mut encode_buffer = Uuid::encode_buffer();
            return serializer.serialize_str(
                self.as_uuid()
                    .as_hyphenated()
                    .encode_lower(&mut encode_buffer),
            );
        }

        // if it's not human readable..
        match self {
            UuidShell::RawUuid(tag) => <Uuid as serde::Serialize>::serialize(tag, serializer),
            UuidShell::Tagged(tag) => tag.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for UuidShell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = if deserializer.is_human_readable() {
            Uuid::deserialize(deserializer).map(UuidShell::RawUuid)?
        } else {
            UuidTag::deserialize(deserializer).map(|v| v.into_shell())?
        };

        Ok(id.find_tag())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UuidTag {
    Team(u16),
    Player(u16),
    Game(u16),
    Raw(Uuid),
}

impl UuidTag {
    pub fn as_uuid(&self) -> Uuid {
        match self {
            UuidTag::Team(val) => TEAM_ID_TABLE.inverter[*val as usize],
            UuidTag::Player(val) => PLAYER_ID_TABLE.inverter[*val as usize],
            UuidTag::Game(val) => GAME_ID_TABLE.inverter[*val as usize],
            UuidTag::Raw(r) => *r,
        }
    }

    pub fn into_shell(self) -> UuidShell {
        match self {
            UuidTag::Raw(r) => UuidShell::RawUuid(r),
            s => UuidShell::Tagged(s),
        }
    }

    pub fn raw_tag(&self) -> Option<u16> {
        match self {
            UuidTag::Team(v) => Some(*v),
            UuidTag::Player(v) => Some(*v),
            UuidTag::Game(v) => Some(*v),
            UuidTag::Raw(_) => None,
        }
    }
}

// impl Serialize for UuidTag {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             UuidTag::Team(val) => serializer.serialize_u32(u32::from_ne_bytes(
//                 PackedTag::new()
//                     .with_tag_val(*val)
//                     .with_kind(PackedTagKind::Team)
//                     .into_bytes(),
//             )),
//             UuidTag::Player(val) => serializer.serialize_u32(u32::from_ne_bytes(
//                 PackedTag::new()
//                     .with_tag_val(*val)
//                     .with_kind(PackedTagKind::Player)
//                     .into_bytes(),
//             )),
//             UuidTag::Game(val) => serializer.serialize_u32(u32::from_ne_bytes(
//                 PackedTag::new()
//                     .with_tag_val(*val)
//                     .with_kind(PackedTagKind::Game)
//                     .into_bytes(),
//             )),
//         }
//     }
// }

// #[bitfield]
// struct PackedTag {
//     tag_val: B30,
//     kind: PackedTagKind,
// }

// #[derive(BitfieldSpecifier)]
// #[bits = 2]
// enum PackedTagKind {
//     Player,
//     Game,
//     Team,
//     Other,
// }

// struct UuidShellVisitor;

// impl<'de> Visitor<'de> for UuidShellVisitor {
//     type Value = UuidShell;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("a uuid tag (u32))")
//     }

//     // fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
//     // where
//     //     E: de::Error,
//     // {
//     //     Ok(UuidShell::Tagged(UuidTag::Team(value)))
//     // }

//     fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         let tag = PackedTag::from_bytes(value.to_ne_bytes());

//         Ok(UuidShell::Tagged(match tag.kind() {
//             PackedTagKind::Player => UuidTag::Player(tag.tag_val()),
//             PackedTagKind::Game => UuidTag::Game(tag.tag_val()),
//             PackedTagKind::Team => UuidTag::Team(tag.tag_val()),
//             _ => unreachable!(),
//         }))
//     }

//     fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         value
//             .parse::<Uuid>()
//             .map(UuidShell::RawUuid)
//             .map_err(|e| E::custom(e))
//     }

//     fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
//         Uuid::from_slice(value)
//             .map(UuidShell::RawUuid)
//             .map_err(|e| E::custom(e))
//     }
// }

// thanks https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398 !
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    if de.is_human_readable() {
        let opt = Option::<String>::deserialize(de)?;
        let opt = opt.as_deref();
        match opt {
            None | Some("") => Ok(None),
            Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        }
    } else {
        Option::<T>::deserialize(de)
    }
}
