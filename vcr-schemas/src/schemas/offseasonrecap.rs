
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Offseasonrecap {
    pub bonus_results: Vec<String>,

    pub decree_results: Vec<String>,

    pub event_results: Vec<String>,

    pub id: String,

    pub name: String,

    pub season: i64,

    pub sim: Option<String>,

    pub total_bonus_votes: i64,

    pub total_decree_votes: i64,

    pub total_will_votes: Option<i64>,

    pub vote_count: i64,

    pub will_results: Option<Vec<String>>,
}
