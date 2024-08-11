use std::{
    collections::{btree_map, BTreeMap},
    ops::RangeBounds,
};

use arrayvec::ArrayVec;
use vcr_lookups::{UuidShell, UuidTag};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct EventIdChunk {
    pub chunk: u16,
    pub ids: ArrayVec<u8, 256>,
}

pub type IdIndex = BTreeMap<i64, EventIdChunk>;

#[derive(Default, Clone)]
pub struct IdIndexBuilder {
    chunks: Vec<(i64, EventIdChunk)>,
}

impl IdIndexBuilder {
    pub fn add(&mut self, time: i64, chunk: u16, id: u8) {
        match self.chunks.binary_search_by_key(&chunk, |k| k.1.chunk) {
            Ok(idx) => {
                let entry = &mut self.chunks[idx];
                entry.0 = std::cmp::min(entry.0, time);
                entry.1.ids.push(id);
            }
            Err(_) => {
                self.chunks.push((
                    time,
                    EventIdChunk {
                        chunk,
                        ids: ArrayVec::from_iter([id]),
                    },
                ));
            }
        }
    }

    pub fn finish(self) -> IdIndex {
        BTreeMap::from_iter(self.chunks.into_iter())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct FeedIndex {
    entities: Vec<IdIndex>,
}

pub struct FeedIndexBuilder {
    entities: Vec<IdIndexBuilder>,
}

impl FeedIndexBuilder {
    pub fn create(capacity: usize) -> FeedIndexBuilder {
        FeedIndexBuilder {
            entities: vec![IdIndexBuilder::default(); capacity],
        }
    }

    pub fn add(&mut self, entity: usize, time: i64, chunk: u16, id: u8) {
        let entity_chunks = &mut self.entities[entity];
        entity_chunks.add(time, chunk, id);
    }

    pub fn finish(self) -> FeedIndex {
        FeedIndex {
            entities: self.entities.into_iter().map(|v| v.finish()).collect(),
        }
    }
}

impl FeedIndex {
    pub fn get<R: RangeBounds<i64>>(
        &self,
        entity: usize,
        range: R,
    ) -> Option<btree_map::Range<'_, i64, EventIdChunk>> {
        self.entities.get(entity).map(|map| map.range(range))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct FeedIndexCollection {
    pub games: FeedIndex,
    pub teams: FeedIndex,
    pub players: FeedIndex,
}

impl FeedIndexCollection {
    pub fn by_game(
        &self,
        game: impl Into<UuidShell>,
        time: impl RangeBounds<i64>,
    ) -> Option<btree_map::Range<'_, i64, EventIdChunk>> {
        let UuidShell::Tagged(UuidTag::Game(id)) = game.into() else {
            return None;
        };

        self.games.get(id as usize, time)
    }

    pub fn by_team(
        &self,
        team: impl Into<UuidShell>,
        time: impl RangeBounds<i64>,
    ) -> Option<btree_map::Range<'_, i64, EventIdChunk>> {
        let UuidShell::Tagged(UuidTag::Team(id)) = team.into() else {
            return None;
        };

        self.teams.get(id as usize, time)
    }

    pub fn by_player(
        &self,
        player: impl Into<UuidShell>,
        time: impl RangeBounds<i64>,
    ) -> Option<btree_map::Range<'_, i64, EventIdChunk>> {
        let UuidShell::Tagged(UuidTag::Player(id)) = player.into() else {
            return None;
        };
        self.players.get(id as usize, time)
    }
}
