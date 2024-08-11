use super::*;
use crate::*;
use blaseball_vcr::db_manager::*;
use blaseball_vcr::etypes::DynamicEntity;
use blaseball_vcr::*;
use rocket::serde::json::Json as RocketJSON;
use rocket::State;
use std::collections::HashSet;
use uuid::Uuid;
use vcr_lookups::{
    DATES_TO_GAMES, GAME_ID_TABLE, PITCHER_TO_GAMES, PLAYER_ID_TABLE, TEAMS_TO_GAMES,
    TEAM_ID_TABLE, WEATHER_TO_GAMES,
};
use vcr_schemas::GameUpdate;

use xxhash_rust::xxh3::Xxh3Builder;

type XxSet<T> = std::collections::HashSet<T, Xxh3Builder>;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamesResponse {
    data: Vec<FinishedGame>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FinishedGame {
    game_id: Uuid,
    start_time: Option<iso8601_timestamp::Timestamp>,
    end_time: Option<iso8601_timestamp::Timestamp>,
    data: GameUpdate,
}

macro_rules! filter_by_table {
    ($parser:expr, $game_tags_out:ident, $list_exp:expr, $id_mapper:ident, $game_table:ident) => {
        if let Some(list) = $list_exp {
            for item in list
                .split_terminator(',')
                .filter_map(|id| $parser(id).ok().and_then(|v| $id_mapper.map(&v)))
            {
                if let Some(games) = $game_table.get(item) {
                    $game_tags_out.retain(|tag| games.contains(tag));
                }
            }
        }
    };

    ($parser:expr, $game_tags_out:ident, $list_exp:expr, $game_table:ident) => {
        if let Some(list) = $list_exp {
            for item in list.split_terminator(',').filter_map(|id| $parser(id).ok()) {
                if let Some(games) = $game_table.get(&item) {
                    $game_tags_out.retain(|tag| games.contains(tag));
                }
            }
        }
    };
}

macro_rules! contents_eq_or_is_none {
    ($lhs:expr, $rhs:expr) => {
        match $lhs {
            Some(v) => v == $rhs,
            None => true,
        }
    };
}

#[get("/v1/games?<req..>")]
pub fn games(
    req: GamesReq<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> VCRResult<RocketJSON<GamesResponse>> {
    let game_db = db_manager
        .get_db::<GameUpdate>()
        .ok_or(VCRError::EntityTypeNotFound)?
        .as_any()
        .downcast_ref::<blaseball_vcr::vhs::db::Database<GameUpdate>>()
        .unwrap();

    let ids = filter_games(&req, game_db);

    let games = game_db
        .get_entities(&ids, req.before_nanos().unwrap_or(i64::MAX))?
        .into_iter()
        .filter_map(|game| {
            game.map(|game| FinishedGame {
                game_id: Uuid::from_bytes(game.entity_id),
                start_time: game_db.index[&game.entity_id]
                    .times
                    .first()
                    .copied()
                    .map(timestamp_from_nanos),
                end_time: game_db.index[&game.entity_id]
                    .times
                    .last()
                    .copied()
                    .map(timestamp_from_nanos),
                data: game.data,
            })
        })
        .collect();

    Ok(RocketJSON(GamesResponse { data: games }))
}

#[get("/v1/games/updates?<req..>")]
pub fn game_updates(
    req: GamesReq<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
    page_manager: &State<PageManager>,
) -> VCRResult<RocketJSON<ChronV1Response<GameUpdateWrapper<DynamicEntity>>>> {
    if let Some(page_token) = req
        .page
        .as_ref()
        .and_then(|v| u64::from_str_radix(v, 16).ok())
    {
        let page_mutex = page_manager
            .get_page(&page_token)
            .ok_or(VCRError::InvalidPageToken)?;
        let mut page = page_mutex.lock();
        let data = page
            .take_n::<GameUpdate>(db_manager, req.count.unwrap_or(100))?
            .into_iter()
            .map(|v| v.as_game_update())
            .collect();

        Ok(RocketJSON(ChronV1Response {
            next_page: if page.is_empty() {
                None
            } else {
                Some(req.page.unwrap())
            },
            data,
        }))
    } else {
        let before = req.before_nanos().unwrap_or(i64::MAX);
        let after = req.after_nanos().unwrap_or(0);

        let ids = if let Some(id_list) = req.game {
            id_list
                .split_terminator(',')
                .filter_map(|id_s| Uuid::parse_str(id_s).ok().map(|v| *v.as_bytes()))
                .collect()
        } else {
            let game_db = db_manager
                .get_db::<GameUpdate>()
                .ok_or(VCRError::EntityTypeNotFound)?
                .as_any()
                .downcast_ref::<blaseball_vcr::vhs::db::Database<GameUpdate>>()
                .unwrap();

            filter_games(&req, game_db)
        };

        let mut page = Page::versions(before, after, ids);
        let data: Vec<GameUpdateWrapper<DynamicEntity>> = page
            .take_n::<GameUpdate>(db_manager, req.count.unwrap_or(100))?
            .into_iter()
            .map(|v| v.as_game_update())
            .collect();

        // if the page isn't empty, add it to the manager
        let token = if !page.is_empty() {
            Some(page_manager.add_page(page))
        } else {
            None
        };

        Ok(RocketJSON(ChronV1Response {
            next_page: token.map(|v| format!("{:X}", v)),
            data,
        }))
    }
}

fn filter_games(
    req: &GamesReq<'_>,
    db: &blaseball_vcr::vhs::db::Database<GameUpdate>,
) -> Vec<[u8; 16]> {
    let before = req.before_nanos();
    let after = req.after_nanos();

    let game_ids: Vec<Uuid> = db
        .index
        .values()
        .filter_map(|game_header| {
            if game_header.times.is_empty() {
                return None;
            }

            if let Some(before) = before {
                // if the first game update we have on file is after the Before parameter, filter it out
                if game_header.times[0] > before {
                    return None;
                }
            }

            if let Some(after) = after {
                // if the last game update we have on file is before the After timestamp, filter it out
                if *game_header.times.last().unwrap() < after {
                    return None;
                }
            }

            Some(Uuid::from_bytes(game_header.id))
        })
        .collect();

    let games_that_match_date: XxSet<u16> =
        if req.tournament.is_some() || req.day.is_some() || req.season.is_some() {
            let mut hash_set =
                HashSet::with_capacity_and_hasher(game_ids.len(), Xxh3Builder::new());
            hash_set.extend(
                DATES_TO_GAMES
                    .keys_values()
                    .filter_map(|(date, games)| {
                        let date = GameDate::from_bytes(*date);

                        if contents_eq_or_is_none!(req.tournament, date.tournament)
                            && contents_eq_or_is_none!(req.day, date.day)
                            && contents_eq_or_is_none!(req.season, date.season)
                        {
                            Some(games)
                        } else {
                            None
                        }
                    })
                    .flatten(),
            );
            hash_set
        } else {
            XxSet::with_capacity_and_hasher(0, Xxh3Builder::new())
        };

    let mut game_tags: Vec<u16> = game_ids
        .iter()
        .map(|id| GAME_ID_TABLE.mapper[id])
        .filter(|tag| {
            if games_that_match_date.is_empty() {
                true
            } else {
                games_that_match_date.contains(tag)
            }
        })
        .collect();

    filter_by_table!(
        Uuid::parse_str,
        game_tags,
        req.pitcher,
        PLAYER_ID_TABLE,
        PITCHER_TO_GAMES
    );
    filter_by_table!(
        Uuid::parse_str,
        game_tags,
        req.team,
        TEAM_ID_TABLE,
        TEAMS_TO_GAMES
    );
    filter_by_table!(
        <u8 as std::str::FromStr>::from_str,
        game_tags,
        req.weather,
        WEATHER_TO_GAMES
    );

    game_tags
        .into_iter()
        .map(|tag| *GAME_ID_TABLE.inverter[tag as usize].as_bytes())
        .collect()
}
