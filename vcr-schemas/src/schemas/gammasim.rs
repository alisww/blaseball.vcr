
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Gammasim {
    pub day_number: i64,

    pub is_offseason: Option<bool>,

    pub is_title_match: Option<bool>,

    pub is_tournament: bool,

    pub last_incineration_time: Option<String>,

    pub live_games: bool,

    pub next_phase_name: Option<String>,

    pub next_phase_start: Option<String>,

    pub next_tournament_phase_start: Option<String>,

    pub phase_end: Option<String>,

    pub phase_end_day: i64,

    pub phase_name: Option<String>,

    pub phase_start: Option<String>,

    pub phase_start_day: i64,

    pub phase_type: Option<String>,

    pub season_number: i64,

    pub show_election_results: Option<bool>,

    pub sim_end: String,

    pub sim_start: String,

    pub total_days_in_season: Option<i64>,

    pub tournament_number: Option<i64>,

    pub tournament_round: Option<i64>,
}
