use serde::{Serialize, Deserialize};
use vcr_lookups::UuidShell;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Diff, vhs_diff::Patch, Debug)]
#[serde(deny_unknown_fields)]
pub struct GameUpdate {
    #[serde(rename = "_id")]
    pub id: Option<UuidShell>,

    #[serde(rename = "atBatBalls")]
    pub at_bat_balls: i64,

    #[serde(rename = "atBatStrikes")]
    pub at_bat_strikes: i64,

    #[serde(rename = "awayBalls")]
    pub away_balls: Option<i64>,

    #[serde(rename = "awayBases")]
    pub away_bases: Option<i64>,

    #[serde(rename = "awayBatter", deserialize_with = "vcr_lookups::empty_string_as_none")]
    pub away_batter: Option<UuidShell>,

    #[serde(rename = "awayBatterMod")]
    pub away_batter_mod: Option<String>,

    #[serde(rename = "awayBatterName")]
    pub away_batter_name: Option<String>,

    #[serde(rename = "awayOdds")]
    pub away_odds: Option<f64>,

    #[serde(rename = "awayOuts")]
    pub away_outs: Option<i64>,

    #[serde(rename = "awayPitcher", deserialize_with = "vcr_lookups::empty_string_as_none")]
    pub away_pitcher: Option<UuidShell>,

    #[serde(rename = "awayPitcherMod")]
    pub away_pitcher_mod: Option<String>,

    #[serde(rename = "awayPitcherName")]
    pub away_pitcher_name: Option<String>,

    #[serde(rename = "awayScore")]
    pub away_score: Option<f64>,

    #[serde(rename = "awayStrikes")]
    pub away_strikes: Option<i64>,

    #[serde(rename = "awayTeam")]
    pub away_team: UuidShell,

    #[serde(rename = "awayTeamBatterCount")]
    pub away_team_batter_count: Option<i64>,

    #[serde(rename = "awayTeamColor")]
    pub away_team_color: String,

    #[serde(rename = "awayTeamEmoji")]
    pub away_team_emoji: String,

    #[serde(rename = "awayTeamName")]
    pub away_team_name: String,

    #[serde(rename = "awayTeamNickname")]
    pub away_team_nickname: String,

    #[serde(rename = "awayTeamRuns")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub away_team_runs: Option<serde_json::Value>,

    #[serde(rename = "awayTeamSecondaryColor")]
    pub away_team_secondary_color: Option<String>,

    #[serde(rename = "baserunnerCount")]
    pub baserunner_count: i64,

    #[serde(rename = "baseRunnerMods")]
    pub base_runner_mods: Option<Vec<String>>,

    #[serde(rename = "baseRunnerNames")]
    pub base_runner_names: Option<Vec<String>>,

    #[serde(rename = "baseRunners")]
    pub base_runners: Vec<UuidShell>,

    #[serde(rename = "basesOccupied")]
    pub bases_occupied: Vec<i64>,

    #[serde(rename = "bottomInningScore")]
    pub bottom_inning_score: Option<f64>,

    #[serde(rename = "day")]
    pub day: i64,

    #[serde(rename = "endPhase")]
    pub end_phase: Option<i64>,

    #[serde(rename = "finalized")]
    pub finalized: Option<bool>,

    #[serde(rename = "gameComplete")]
    pub game_complete: bool,

    #[serde(rename = "gameStart")]
    pub game_start: bool,

    #[serde(rename = "gameStartPhase")]
    pub game_start_phase: Option<i64>,

    #[serde(rename = "halfInningOuts")]
    pub half_inning_outs: i64,

    #[serde(rename = "halfInningScore")]
    pub half_inning_score: f64,

    #[serde(rename = "homeBalls")]
    pub home_balls: Option<i64>,

    #[serde(rename = "homeBases")]
    pub home_bases: Option<i64>,

    #[serde(rename = "homeBatter", deserialize_with = "vcr_lookups::empty_string_as_none")]
    pub home_batter: Option<UuidShell>,

    #[serde(rename = "homeBatterMod")]
    pub home_batter_mod: Option<String>,

    #[serde(rename = "homeBatterName")]
    pub home_batter_name: Option<String>,

    #[serde(rename = "homeOdds")]
    pub home_odds: Option<f64>,

    #[serde(rename = "homeOuts")]
    pub home_outs: Option<i64>,

    #[serde(rename = "homePitcher", deserialize_with = "vcr_lookups::empty_string_as_none")]
    pub home_pitcher: Option<UuidShell>,

    #[serde(rename = "homePitcherMod")]
    pub home_pitcher_mod: Option<String>,

    #[serde(rename = "homePitcherName")]
    pub home_pitcher_name: Option<String>,

    #[serde(rename = "homeScore")]
    pub home_score: Option<f64>,

    #[serde(rename = "homeStrikes")]
    pub home_strikes: Option<i64>,

    #[serde(rename = "homeTeam")]
    pub home_team: UuidShell,

    #[serde(rename = "homeTeamBatterCount")]
    pub home_team_batter_count: i64,

    #[serde(rename = "homeTeamColor")]
    pub home_team_color: String,

    #[serde(rename = "homeTeamEmoji")]
    pub home_team_emoji: String,

    #[serde(rename = "homeTeamName")]
    pub home_team_name: String,

    #[serde(rename = "homeTeamNickname")]
    pub home_team_nickname: String,

    #[serde(rename = "homeTeamRuns")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub home_team_runs: Option<serde_json::Value>,

    #[serde(rename = "homeTeamSecondaryColor")]
    pub home_team_secondary_color: Option<String>,

    #[serde(rename = "id")]
    pub games_schema_id: Option<String>,

    #[serde(rename = "inning")]
    pub inning: i64,

    #[serde(rename = "isPostseason")]
    pub is_postseason: bool,

    #[serde(rename = "isPrizeMatch")]
    pub is_prize_match: Option<bool>,

    #[serde(rename = "isTitleMatch")]
    pub is_title_match: Option<bool>,

    #[serde(rename = "lastUpdate")]
    pub last_update: Option<String>,

    #[serde(rename = "lastUpdateFull")]
    pub last_update_full: Option<Vec<LastUpdateFull>>,

    #[serde(rename = "loser")]
    pub loser: Option<String>,

    #[serde(rename = "newHalfInningPhase")]
    pub new_half_inning_phase: Option<i64>,

    #[serde(rename = "newInningPhase")]
    pub new_inning_phase: Option<i64>,

    #[serde(rename = "outcomes")]
    pub outcomes: Option<Vec<String>>,

    #[serde(rename = "phase")]
    pub phase: i64,

    #[serde(rename = "playCount")]
    pub play_count: Option<i64>,

    #[serde(rename = "queuedEvents")]
    pub queued_events: Option<Vec<QueuedEvent>>,

    #[serde(rename = "repeatCount")]
    pub repeat_count: Option<i64>,

    #[serde(rename = "rules")]
    pub rules: Option<String>,

    #[serde(rename = "scoreLedger")]
    pub score_ledger: Option<String>,

    #[serde(rename = "scoreUpdate")]
    pub score_update: Option<String>,

    #[serde(rename = "season")]
    pub season: i64,

    #[serde(rename = "seasonId")]
    pub season_id: Option<String>,

    #[serde(rename = "secretBaserunner")]
    pub secret_baserunner: Option<String>,

    #[serde(rename = "seriesIndex")]
    pub series_index: i64,

    #[serde(rename = "seriesLength")]
    pub series_length: i64,

    #[serde(rename = "shame")]
    pub shame: bool,

    #[serde(rename = "sim")]
    pub sim: Option<String>,

    #[serde(rename = "stadiumId")]
    pub stadium_id: Option<String>,

    #[serde(rename = "state")]
    pub state: Option<State>,

    #[serde(rename = "statsheet")]
    pub statsheet: Option<String>,

    #[serde(rename = "terminology")]
    pub terminology: Option<String>,

    #[serde(rename = "topInningScore")]
    pub top_inning_score: Option<f64>,

    #[serde(rename = "topOfInning")]
    pub top_of_inning: bool,

    #[serde(rename = "tournament")]
    pub tournament: Option<i64>,

    #[serde(rename = "tournamentRound")]
    pub tournament_round: Option<i64>,

    #[serde(rename = "tournamentRoundGameIndex")]
    pub tournament_round_game_index: Option<i64>,

    #[serde(rename = "weather")]
    pub weather: Option<i64>,

    #[serde(rename = "winner")]
    pub winner: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct LastUpdateFull {
    #[serde(rename = "blurb")]
    pub blurb: String,

    #[serde(rename = "category")]
    pub category: i64,

    #[serde(rename = "created")]
    pub created: String,

    #[serde(rename = "day")]
    pub day: i64,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "gameTags")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_vec")]

    pub game_tags: Vec<serde_json::Value>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "nuts")]
    pub nuts: i64,

    #[serde(rename = "phase")]
    pub phase: i64,

    #[serde(rename = "playerTags")]
    pub player_tags: Vec<String>,

    #[serde(rename = "season")]
    pub season: i64,

    #[serde(rename = "teamTags")]
    pub team_tags: Vec<String>,

    #[serde(rename = "tournament")]
    pub tournament: i64,

    #[serde(rename = "type")]
    pub last_update_full_type: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "after")]
    pub after: Option<f64>,

    #[serde(rename = "aLocation")]
    pub a_location: Option<i64>,

    #[serde(rename = "amount")]
    pub amount: Option<i64>,

    #[serde(rename = "aPlayerId")]
    pub a_player_id: Option<String>,

    #[serde(rename = "aPlayerName")]
    pub a_player_name: Option<String>,

    #[serde(rename = "aTeamId")]
    pub a_team_id: Option<String>,

    #[serde(rename = "aTeamName")]
    pub a_team_name: Option<String>,

    #[serde(rename = "away")]
    pub away: Option<String>,

    #[serde(rename = "awayEmoji")]
    pub away_emoji: Option<String>,

    #[serde(rename = "awayScore")]
    pub away_score: Option<i64>,

    #[serde(rename = "before")]
    pub before: Option<f64>,

    #[serde(rename = "bLocation")]
    pub b_location: Option<i64>,

    #[serde(rename = "bPlayerId")]
    pub b_player_id: Option<String>,

    #[serde(rename = "bPlayerName")]
    pub b_player_name: Option<String>,

    #[serde(rename = "bTeamId")]
    pub b_team_id: Option<String>,

    #[serde(rename = "bTeamName")]
    pub b_team_name: Option<String>,

    #[serde(rename = "effect")]
    pub effect: Option<String>,

    #[serde(rename = "home")]
    pub home: Option<String>,

    #[serde(rename = "homeEmoji")]
    pub home_emoji: Option<String>,

    #[serde(rename = "homeScore")]
    pub home_score: Option<i64>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "inPlayerId")]
    pub in_player_id: Option<String>,

    #[serde(rename = "inPlayerName")]
    pub in_player_name: Option<String>,

    #[serde(rename = "ledger")]
    pub ledger: Option<String>,

    #[serde(rename = "lines")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_vecopt")]
    pub lines: Option<Vec<serde_json::Value>>,

    #[serde(rename = "location")]
    pub location: Option<i64>,

    #[serde(rename = "mod")]
    pub metadata_mod: Option<String>,

    #[serde(rename = "outPlayerId")]
    pub out_player_id: Option<String>,

    #[serde(rename = "outPlayerName")]
    pub out_player_name: Option<String>,

    #[serde(rename = "source")]
    pub source: Option<String>,

    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(rename = "teamName")]
    pub team_name: Option<String>,

    #[serde(rename = "type")]
    pub metadata_type: Option<i64>,

    #[serde(rename = "update")]
    pub update: Option<String>,

    #[serde(rename = "weather")]
    pub weather: Option<i64>,

    #[serde(rename = "winner")]
    pub winner: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueuedEvent {
    #[serde(rename = "delay")]
    pub delay: i64,

    #[serde(rename = "isSpecial")]
    pub is_special: bool,

    #[serde(rename = "logUpdates")]
    pub log_updates: Vec<String>,

    #[serde(rename = "outcomes")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_vec")]
    pub outcomes: Vec<serde_json::Value>,

    #[serde(rename = "type")]
    pub queued_event_type: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    #[serde(rename = "ego_player_data")]
    pub ego_player_data: Option<Vec<EgoPlayerDatum>>,

    #[serde(rename = "game_cancelled")]
    pub game_cancelled: Option<bool>,

    #[serde(rename = "holidayInning")]
    pub holiday_inning: Option<bool>,

    #[serde(rename = "postseason")]
    pub postseason: Option<Postseason>,

    #[serde(rename = "prizeMatch")]
    pub prize_match: Option<PrizeMatch>,

    #[serde(rename = "reload")]
    pub reload: Option<Reload>,

    #[serde(rename = "snowfallEvents")]
    pub snowfall_events: Option<i64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct EgoPlayerDatum {
    #[serde(rename = "hallPlayer")]
    pub hall_player: bool,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "location")]
    pub location: Option<i64>,

    #[serde(rename = "team")]
    pub team: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Postseason {
    #[serde(rename = "bracket")]
    pub bracket: i64,

    #[serde(rename = "matchup")]
    pub matchup: String,

    #[serde(rename = "playoffId")]
    pub playoff_id: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PrizeMatch {
    #[serde(rename = "itemId")]
    pub item_id: String,

    #[serde(rename = "itemName")]
    pub item_name: String,

    #[serde(rename = "winner")]
    pub winner: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Reload {
    #[serde(rename = "BaserunnerCount")]
    pub baserunner_count: i64,

    #[serde(rename = "Baserunners")]
    pub baserunners: Vec<String>,

    #[serde(rename = "BasesOccupied")]
    pub bases_occupied: Vec<i64>,

    #[serde(rename = "Batter")]
    pub batter: String,
}
