use bitcode::{Decode, Encode};
use borsh::BorshSerialize;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use uuid::Uuid;
use vcr_schemas::{
    Bossfight, CommunityChestProgress, Division, GameUpdate, League, Playoffmatchup, Playoffround,
    Playoffs, Season, Sim, Stadium, Standings, Subleague, Sunsun, Team, Temporal, Tiebreakers,
    Tournament,
};
use xxhash_rust::xxh3::xxh3_128;

use crate::{
    db_manager::DatabaseManager, pack_entities, unpack_entities, EntityLocation, VCRResult
};

use super::{PackedStreamComponent, StreamComponent};

#[derive(Deserialize, Serialize)]
pub struct StreamDataWrapper {
    pub value: Option<StreamData>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamData {
    #[serde(default)]
    pub fights: Option<StreamDataBossFights>,
    #[serde(default)]
    pub games: Option<StreamDataGames>,
    #[serde(default)]
    pub temporal: Option<Temporal>,
    #[serde(default)]
    pub leagues: Option<StreamDataLeagues>,
}

#[derive(Encode, Decode)]
pub struct PackedStreamData {
    pub fights: Option<PackedBossFights>,
    pub games: Option<PackedStreamGames>,
    pub temporal: Option<EntityLocation>,
    pub leagues: Option<PackedStreamLeagues>,
}

impl StreamComponent for StreamData {
    type Packed = PackedStreamData;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> VCRResult<Self::Packed> {
        Ok(PackedStreamData {
            fights: self
                .fights
                .as_ref()
                .map(|v| v.pack(time, location_table, database_manager))
                .transpose()?,
            games: self
                .games
                .as_ref()
                .map(|v| v.pack(time, location_table, database_manager))
                .transpose()?,
            temporal: pack_entities!(one of Temporal, &self.temporal, location_table),
            leagues: self
                .leagues
                .as_ref()
                .map(|v| v.pack(time, location_table, database_manager))
                .transpose()?,
        })
    }
}

impl PackedStreamComponent for PackedStreamData {
    type Unpacked = StreamData;

    fn unpack(
        &self,
        time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> VCRResult<Self::Unpacked> {
        Ok(StreamData {
            fights: self
                .fights
                .as_ref()
                .map(|v| v.unpack(time, database))
                .transpose()?,
            games: self
                .games
                .as_ref()
                .map(|v| v.unpack(time, database))
                .transpose()?,
            temporal: unpack_entities!(one of Temporal, time, &self.temporal, database),
            leagues: self
                .leagues
                .as_ref()
                .map(|v| v.unpack(time, database))
                .transpose()?,
        })
    }
}

#[derive(Deserialize, Serialize, Derivative, Debug)]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamDataGames {
    #[serde(default)]
    pub season: Option<Season>,
    #[serde(default)]
    #[derivative(PartialEq = "ignore")]
    pub postseason: Option<StreamDataPostSeason>,
    #[serde(default)]
    pub postseasons: Option<Vec<StreamDataPostSeason>>,
    #[serde(default)]
    pub standings: Option<Standings>,
    #[serde(default)]
    pub sim: Option<Sim>,
    #[serde(rename = "tomorrowSchedule", default)]
    #[derivative(PartialEq = "ignore")]
    pub tomorrow_schedule: Option<Vec<GameUpdate>>,
    #[serde(default)]
    pub schedule: Option<Vec<GameUpdate>>,
    #[serde(default)]
    pub tournament: Option<Tournament>,
    #[serde(rename = "clientMeta", default)]
    pub client_meta: Option<ClientMeta>,
    #[serde(default)]
    pub last_update_time: Option<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Encode, Decode, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClientMeta {
    #[serde(default)]
    process_id: Option<String>,
    #[serde(default)]
    timestamp: Option<i64>,
    #[serde(default)]
    last_event_id: Option<String>
}

#[derive(Encode, Decode)]
pub struct PackedStreamGames {
    season: Option<EntityLocation>,
    postseason: Option<PackedStreamPostSeason>,
    postseasons: Option<Vec<PackedStreamPostSeason>>,
    standings: Option<EntityLocation>,
    sim: Option<EntityLocation>,
    pub schedule: Option<Vec<EntityLocation>>,
    tomorrow_schedule: Option<Vec<EntityLocation>>,
    tournament: Option<EntityLocation>,
    client_meta: Option<ClientMeta>,
    last_update_time: Option<i64>,
}

impl StreamComponent for StreamDataGames {
    type Packed = PackedStreamGames;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> crate::VCRResult<Self::Packed> {
        Ok(PackedStreamGames {
            season: pack_entities!(one of Season, fallback, time, &self.season, database_manager, location_table, |s: &Season| s.id().unwrap()),
            postseason: self
                .postseason
                .as_ref()
                .map(|v| v.pack(time, location_table, database_manager))
                .transpose()?,
            postseasons: self
                .postseasons
                .as_ref()
                .map(|v| {
                    v.into_iter()
                        .map(|p| p.pack(time, location_table, database_manager))
                        .collect::<VCRResult<Vec<PackedStreamPostSeason>>>()
                })
                .transpose()?,
            standings: pack_entities!(one of Standings, fallback, time, &self.standings, database_manager, location_table, |v: &Standings| v.id()),
            sim: pack_entities!(one of Sim, fallback, time, &self.sim, database_manager, location_table, |_: &Sim| Uuid::nil()),
            tournament: pack_entities!(one of Tournament, fallback, time, &self.tournament, database_manager, location_table, |t: &Tournament| t.id.parse::<Uuid>().unwrap()),
            tomorrow_schedule: pack_entities!(by id list of GameUpdate, time, &self.tomorrow_schedule, database_manager, { |game: &GameUpdate| game.id.map(|id| id.as_uuid()).unwrap_or_else(|| game.games_schema_id.as_ref().unwrap().parse::<Uuid>().unwrap()) }),
            // tomorrow_schedule: self.tomorrow_schedule.as_ref().map(|v| {
            //     let mut positions = Vec::new();
            //     let db = database_manager.get_db::<GameUpdate>().unwrap();
            //     for game in v {
            // let id = game.id.map(|id| id.as_uuid())
            // .unwrap_or_else(|| game.games_schema_id.as_ref().unwrap().parse::<Uuid>().unwrap());
            //         positions.push(db.index_from_id(id.as_bytes()).unwrap() );
            //     }

            //     positions
            // }),
            schedule: pack_entities!(list of GameUpdate, fallback, time, &self.schedule, database_manager, location_table,  { |game: &GameUpdate| game.id.map(|id| id.as_uuid()).unwrap_or_else(|| game.games_schema_id.as_ref().unwrap().parse::<Uuid>().unwrap()) }),
            client_meta: self.client_meta.clone(),
            last_update_time: self.last_update_time.clone(),
        })
    }
}

impl PackedStreamComponent for PackedStreamGames {
    type Unpacked = StreamDataGames;

    fn unpack(
        &self,
        time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> crate::VCRResult<Self::Unpacked> {
        Ok(StreamDataGames {
            season: unpack_entities!(one of Season, time, &self.season, database),
            postseason: self
                .postseason
                .as_ref()
                .map(|v| v.unpack(time, database))
                .transpose()?,
            postseasons: self
                .postseasons
                .as_ref()
                .map(|v| {
                    v.into_iter()
                        .map(|p| p.unpack(time, database))
                        .collect::<VCRResult<Vec<StreamDataPostSeason>>>()
                })
                .transpose()?,
            standings: unpack_entities!(one of Standings, time, &self.standings, database),
            sim: unpack_entities!(one of Sim, time, &self.sim, database),
            tournament: unpack_entities!(one of Tournament, time, &self.tournament, database),
            tomorrow_schedule: unpack_entities!(list of GameUpdate, 0, &self.tomorrow_schedule, database),
            schedule: unpack_entities!(list of GameUpdate, time, &self.schedule, database),
            client_meta: self.client_meta.clone(),
            last_update_time: self.last_update_time.clone(),
        })
    }
}

#[derive(Deserialize, Serialize, Derivative, Debug)]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamDataLeagues {
    #[serde(default)]
    leagues: Option<Vec<League>>,
    #[serde(default)]
    divisions: Option<Vec<Division>>,
    #[serde(default)]
    teams: Option<Vec<Team>>,
    #[serde(default)]
    #[derivative(PartialEq = "ignore")]
    tiebreakers: Option<Tiebreakers>,
    #[serde(default)]
    stadiums: Option<Vec<Stadium>>,
    #[serde(default)]
    stats: Option<StreamDataStats>,
    #[serde(default)]
    subleagues: Option<Vec<Subleague>>,
}

#[derive(Encode, Decode)]
pub struct PackedStreamLeagues {
    leagues: Option<Vec<EntityLocation>>,
    divisions: Option<Vec<EntityLocation>>,
    teams: Option<Vec<EntityLocation>>,
    tiebreakers: Option<EntityLocation>,
    stadiums: Option<Vec<EntityLocation>>,
    stats: Option<PackedStreamStats>,
    subleagues: Option<Vec<EntityLocation>>,
}

impl StreamComponent for StreamDataLeagues {
    type Packed = PackedStreamLeagues;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> crate::VCRResult<Self::Packed> {
        Ok(PackedStreamLeagues {
            leagues: pack_entities!(list of League, fallback, time, &self.leagues, database_manager, location_table, |l: &League| l.id),
            divisions: pack_entities!(list of Division, fallback, time, &self.divisions, database_manager, location_table, |d: &Division| d.id.parse::<Uuid>().unwrap()),
            teams: pack_entities!(list of Team, fallback, time, &self.teams, database_manager, location_table, |t: &Team| t.id()),
            tiebreakers: pack_entities!(one of Tiebreakers, fallback, time, &self.tiebreakers, database_manager, location_table, |_: &Tiebreakers| Uuid::nil()),
            stadiums: pack_entities!(list of Stadium, fallback, time, &self.stadiums, database_manager, location_table, |s: &Stadium| s.id.parse::<Uuid>().unwrap()),
            stats: self
                .stats
                .as_ref()
                .map(|v| v.pack(time, location_table, database_manager))
                .transpose()?,
            subleagues: pack_entities!(list of Subleague, fallback, time, &self.subleagues, database_manager, location_table, |s: &Subleague| s.id),
        })
    }
}

impl PackedStreamComponent for PackedStreamLeagues {
    type Unpacked = StreamDataLeagues;

    fn unpack(
        &self,
        time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> crate::VCRResult<Self::Unpacked> {
        Ok(StreamDataLeagues {
            leagues: unpack_entities!(list of League, time, &self.leagues, database),
            divisions: unpack_entities!(list of Division, time, &self.divisions, database),
            teams: unpack_entities!(list of Team, time, &self.teams, database),
            tiebreakers: unpack_entities!(one of Tiebreakers, time, &self.tiebreakers, database),
            stadiums: unpack_entities!(list of Stadium, time, &self.stadiums, database),
            stats: self
                .stats
                .as_ref()
                .map(|v| v.unpack(time, database))
                .transpose()?,
            subleagues: unpack_entities!(list of Subleague, time, &self.subleagues, database),
        })
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamDataStats {
    #[serde(default)]
    community_chest: Option<CommunityChestProgress>,
    #[serde(default)]
    sunsun: Option<Sunsun>,
}

#[derive(Encode, Decode)]
pub struct PackedStreamStats {
    community_chest: Option<EntityLocation>,
    sunsun: Option<EntityLocation>,
}

impl StreamComponent for StreamDataStats {
    type Packed = PackedStreamStats;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> crate::VCRResult<Self::Packed> {
        Ok(PackedStreamStats {
            community_chest: pack_entities!(one of CommunityChestProgress, fallback, time, &self.community_chest, database_manager, location_table, |_: &CommunityChestProgress| Uuid::nil()),
            sunsun: pack_entities!(one of Sunsun, fallback, time, &self.sunsun, database_manager, location_table, |_: &Sunsun| Uuid::nil()),
        })
    }
}

impl PackedStreamComponent for PackedStreamStats {
    type Unpacked = StreamDataStats;

    fn unpack(
        &self,
        _time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> crate::VCRResult<Self::Unpacked> {
        Ok(StreamDataStats {
            community_chest: unpack_entities!(one of CommunityChestProgress, time, &self.community_chest, database),
            sunsun: unpack_entities!(one of Sunsun, time, &self.sunsun, database),
        })
    }
}

#[derive(Deserialize, Serialize, Derivative, Debug)]
#[derivative(PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamDataPostSeason {
    #[serde(default)]
    round: Option<Playoffround>,
    #[serde(default)]
    tomorrow_matchups: Option<Vec<Playoffmatchup>>,
    #[serde(default)]
    all_matchups: Option<Vec<Playoffmatchup>>,
    #[serde(default)]
    matchups: Option<Vec<Playoffmatchup>>,
    #[serde(default)]
    playoffs: Option<Playoffs>,
    #[serde(default)]
    #[derivative(PartialEq = "ignore")]
    tomorrow_round: Option<Playoffround>,
    #[serde(default)]
    all_rounds: Option<Vec<Playoffround>>,
}

#[derive(Encode, Decode)]
pub struct PackedStreamPostSeason {
    round: Option<EntityLocation>,
    tomorrow_matchups: Option<Vec<EntityLocation>>,
    all_matchups: Option<Vec<EntityLocation>>,
    matchups: Option<Vec<EntityLocation>>,
    playoffs: Option<EntityLocation>,
    tomorrow_round: Option<EntityLocation>,
    all_rounds: Option<Vec<EntityLocation>>,
}

impl StreamComponent for StreamDataPostSeason {
    type Packed = PackedStreamPostSeason;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> crate::VCRResult<Self::Packed> {
        Ok(PackedStreamPostSeason {
            round: pack_entities!(one of Playoffround, fallback, time, &self.round, database_manager, location_table, |round: &Playoffround| round.id.unwrap()),
            tomorrow_matchups: pack_entities!(by id list of Playoffmatchup, time, &self.tomorrow_matchups, database_manager, |round: &Playoffmatchup| round.id),
            all_matchups: pack_entities!(list of Playoffmatchup,  fallback, time, &self.all_matchups, database_manager, location_table, |matchup: &Playoffmatchup| matchup.id),
            matchups: pack_entities!(list of Playoffmatchup, fallback, time, &self.matchups, database_manager, location_table, |matchup: &Playoffmatchup| matchup.id),
            playoffs: pack_entities!(one of Playoffs, fallback, time, &self.playoffs, database_manager, location_table, |playoffs: &Playoffs| playoffs.id()),
            tomorrow_round: pack_entities!(by id one of Playoffround, time, &self.tomorrow_round, database_manager, |round: &Playoffround| round.id.unwrap()),
            // tomorrow_round: pack_entities!(by id one of Playoffround, time, &self.tomorrow_round, database_manager, |round: &Playoffround| round.id.unwrap()),
            all_rounds: pack_entities!(list of Playoffround, fallback, time, &self.all_rounds, database_manager, location_table, |round: &Playoffround| round.id.unwrap()),
        })
    }
}

impl PackedStreamComponent for PackedStreamPostSeason {
    type Unpacked = StreamDataPostSeason;

    fn unpack(
        &self,
        _time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> crate::VCRResult<Self::Unpacked> {
        Ok(StreamDataPostSeason {
            round: unpack_entities!(one of Playoffround, time, &self.round, database),
            tomorrow_matchups: unpack_entities!(list of Playoffmatchup, time, &self.tomorrow_matchups, database),
            all_matchups: unpack_entities!(list of Playoffmatchup, time, &self.all_matchups, database),
            matchups: unpack_entities!(list of Playoffmatchup, time, &self.matchups, database),
            playoffs: unpack_entities!(one of Playoffs, time, &self.playoffs, database),
            tomorrow_round: unpack_entities!(one of Playoffround, time, &self.tomorrow_round, database),
            all_rounds: unpack_entities!(list of Playoffround, time, &self.all_rounds, database),
        })
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StreamDataBossFights {
    #[serde(default)]
    boss_fights: Option<Vec<Bossfight>>,
}

#[derive(Encode, Decode)]
pub struct PackedBossFights {
    boss_fights: Option<Vec<EntityLocation>>,
}

impl StreamComponent for StreamDataBossFights {
    type Packed = PackedBossFights;

    fn pack(
        &self,
        time: i64,
        location_table: &redb::ReadOnlyTable<u128, crate::EntityLocation>,
        database_manager: &DatabaseManager,
    ) -> crate::VCRResult<Self::Packed> {
        Ok(PackedBossFights {
            boss_fights: pack_entities!(list of Bossfight, fallback, time, &self.boss_fights, database_manager, location_table, |b: &Bossfight| b.id.parse::<Uuid>().unwrap()),
        })
    }
}

impl PackedStreamComponent for PackedBossFights {
    type Unpacked = StreamDataBossFights;

    fn unpack(
        &self,
        _time: i64,
        database: &crate::db_manager::DatabaseManager,
    ) -> crate::VCRResult<Self::Unpacked> {
        Ok(StreamDataBossFights {
            boss_fights: unpack_entities!(list of Bossfight, time, &self.boss_fights, database),
        })
    }
}
