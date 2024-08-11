use std::{cmp, collections::BTreeMap};
use borsh::BorshSerialize;

use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize,
};
use smallvec::SmallVec;
use vcr_lookups::{UuidShell, UuidTag};


#[derive(BorshSerialize, Clone, Copy, PartialEq, Eq)]
// #[serde(deny_unknown_fields)]
pub struct TributeValue(pub i64, pub bool);

impl PartialOrd for TributeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for TributeValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(BorshSerialize, Clone, PartialEq, Eq)]
// #[serde(deny_unknown_fields)]
pub enum Tributes {
    Players(BTreeMap<u16, TributeValue>),
    PlayersAndTeams {
        players: BTreeMap<u16, TributeValue>,
        teams: BTreeMap<u16, TributeValue>,
    },
}

struct SerializablePlayerMap<'a>(&'a BTreeMap<u16, TributeValue>);
impl<'a> Serialize for SerializablePlayerMap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut elements = self
            .0
            .iter()
            .filter_map(
                |(k, TributeValue(v, is_present))| if *is_present { Some((*k, *v)) } else { None },
            )
            .collect::<SmallVec<[(u16, i64); 128]>>();
        let mut seq = serializer.serialize_seq(Some(elements.len()))?;

        elements.sort_unstable_by_key(|(_, k)| cmp::Reverse(*k));

        for (k, v) in elements {
            seq.serialize_element(&SerializablePlayer(k, v))?;
        }

        seq.end()
    }
}
struct SerializableTeamMap<'a>(&'a BTreeMap<u16, TributeValue>);
impl<'a> Serialize for SerializableTeamMap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut elements = self
            .0
            .iter()
            .filter_map(
                |(k, TributeValue(v, is_present))| if *is_present { Some((*k, *v)) } else { None },
            )
            .collect::<SmallVec<[(u16, i64); 128]>>();
        let mut seq = serializer.serialize_seq(Some(elements.len()))?;

        elements.sort_unstable_by_key(|(_, k)| cmp::Reverse(*k));

        for (k, v) in elements {
            seq.serialize_element(&SerializableTeam(k, v))?;
        }

        seq.end()
    }
}

struct SerializableTeam(u16, i64);
impl Serialize for SerializableTeam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Team", 2)?;
        state.serialize_field("teamId", &UuidShell::Tagged(UuidTag::Team(self.0)))?;
        state.serialize_field("peanuts", &self.1)?;
        state.end()
    }
}

struct SerializablePlayer(u16, i64);

impl Serialize for SerializablePlayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Player", 2)?;
        state.serialize_field("playerId", &UuidShell::Tagged(UuidTag::Player(self.0)))?;
        state.serialize_field("peanuts", &self.1)?;
        state.end()
    }
}

impl Serialize for Tributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Tributes::Players(players) => SerializablePlayerMap(players).serialize(serializer),
            Tributes::PlayersAndTeams { players, teams } => {
                let mut state = serializer.serialize_struct("Tributes", 2)?;

                state.serialize_field("players", &SerializablePlayerMap(players))?;
                state.serialize_field("teams", &SerializableTeamMap(teams))?;

                state.end()
            }
        }
    }
}

// #[modular_bitfield::bitfield(bits = 32)]
// #[repr(u32)]
// pub struct TributeCommand {
//     id: B16,
//     #[bits = 1]
//     command_kind: CommandKind,
//     #[bits = 1]
//     affects: TributeAffects,
//     #[skip] __: B14
// }
