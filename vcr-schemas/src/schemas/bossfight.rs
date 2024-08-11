
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Bossfight {
    pub at_bat_balls: i64,

    pub at_bat_strikes: i64,

    pub away_balls: Option<i64>,

    pub away_bases: i64,

    pub away_batter: Option<String>,

    pub away_batter_name: String,

    pub away_hp: String,

    pub away_max_hp: String,

    pub away_odds: f64,

    pub away_outs: Option<i64>,

    pub away_pitcher: String,

    pub away_pitcher_name: String,

    pub away_score: i64,

    pub away_strikes: i64,

    pub away_team: String,

    pub away_team_batter_count: i64,

    pub away_team_color: String,

    pub away_team_emoji: String,

    pub away_team_name: String,

    pub away_team_nickname: String,

    pub away_team_secondary_color: String,

    pub baserunner_count: i64,

    pub base_runner_names: Vec<String>,

    pub base_runners: Vec<String>,

    pub bases_occupied: Vec<i64>,

    pub damage_results: String,

    pub day: i64,

    pub finalized: bool,

    pub game_complete: bool,

    pub game_start: bool,

    pub half_inning_outs: i64,

    pub half_inning_score: i64,

    pub home_balls: Option<i64>,

    pub home_bases: i64,

    pub home_batter: Option<String>,

    pub home_batter_name: String,

    pub home_hp: String,

    pub home_max_hp: String,

    pub home_odds: f64,

    pub home_outs: Option<i64>,

    pub home_pitcher: String,

    pub home_pitcher_name: String,

    pub home_score: i64,

    pub home_strikes: i64,

    pub home_team: String,

    pub home_team_batter_count: i64,

    pub home_team_color: String,

    pub home_team_emoji: String,

    pub home_team_name: String,

    pub home_team_nickname: String,

    pub home_team_secondary_color: String,

    pub id: String,

    pub inning: i64,

    pub is_postseason: bool,

    pub last_update: String,

    pub outcomes: Vec<String>,

    pub phase: i64,

    pub play_count: Option<i64>,

    pub repeat_count: i64,

    pub rules: String,

    pub season: i64,

    pub series_index: i64,

    pub series_length: i64,

    pub shame: bool,

    pub statsheet: String,

    pub terminology: String,

    pub top_of_inning: bool,

    pub weather: i64,
}
