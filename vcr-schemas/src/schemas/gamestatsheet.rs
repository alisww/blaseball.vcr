use serde::*;
use vhs_diff::*;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Diff, Patch, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Gamestatsheet {
    #[serde(rename = "awayTeamRunsByInning")]
    pub away_team_runs_by_inning: Vec<f64>,

    #[serde(rename = "awayTeamStats")]
    pub away_team_stats: String,

    #[serde(rename = "awayTeamTotalBatters")]
    pub away_team_total_batters: i64,

    #[serde(rename = "homeTeamRunsByInning")]
    pub home_team_runs_by_inning: Vec<f64>,

    #[serde(rename = "homeTeamStats")]
    pub home_team_stats: String,

    #[serde(rename = "homeTeamTotalBatters")]
    pub home_team_total_batters: i64,

    #[serde(rename = "id")]
    pub id: String,
}
