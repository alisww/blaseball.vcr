use serde::*;
use vhs_diff::*;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Diff, Patch, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Playerstatsheet {
    #[serde(rename = "atBats")]
    pub at_bats: i64,

    #[serde(rename = "caughtStealing")]
    pub caught_stealing: i64,

    #[serde(rename = "created")]
    pub created: Option<String>,

    #[serde(rename = "doubles")]
    pub doubles: i64,

    #[serde(rename = "earnedRuns")]
    pub earned_runs: f64,

    #[serde(rename = "groundIntoDp")]
    pub ground_into_dp: i64,

    #[serde(rename = "hitBatters")]
    pub hit_batters: i64,

    #[serde(rename = "hitByPitch")]
    pub hit_by_pitch: i64,

    #[serde(rename = "hits")]
    pub hits: i64,

    #[serde(rename = "hitsAllowed")]
    pub hits_allowed: i64,

    #[serde(rename = "homeRuns")]
    pub home_runs: i64,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "losses")]
    pub losses: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "outsRecorded")]
    pub outs_recorded: i64,

    #[serde(rename = "pitchesThrown")]
    pub pitches_thrown: Option<i64>,

    #[serde(rename = "playerId")]
    pub player_id: Option<String>,

    #[serde(rename = "quadruples")]
    pub quadruples: i64,

    #[serde(rename = "rbis")]
    pub rbis: i64,

    #[serde(rename = "runs")]
    pub runs: f64,

    #[serde(rename = "seasonId")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub season_id: Option<serde_json::Value>,

    #[serde(rename = "stolenBases")]
    pub stolen_bases: i64,

    #[serde(rename = "strikeouts")]
    pub strikeouts: i64,

    #[serde(rename = "struckouts")]
    pub struckouts: i64,

    #[serde(rename = "team")]
    pub team: String,

    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(rename = "triples")]
    pub triples: i64,

    #[serde(rename = "walks")]
    pub walks: i64,

    #[serde(rename = "walksIssued")]
    pub walks_issued: i64,

    #[serde(rename = "wins")]
    pub wins: i64,
}
