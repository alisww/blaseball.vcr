
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Playoffround {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(alias = "_id")]
    pub id: Option<Uuid>,

    pub game_index: i64,

    pub games: Vec<Vec<String>>,

    pub matchups: Vec<Uuid>,

    pub name: String,

    pub round_number: i64,

    pub special: bool,

    pub winners: Vec<String>,

    pub winner_seeds: Vec<i64>,
}
