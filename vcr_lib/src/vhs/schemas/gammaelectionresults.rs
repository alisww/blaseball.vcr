
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Gammaelectionresults {
    inner: Vec<Gammaelectionresult>
} 

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Gammaelectionresult {
    pub blurb: String,

    pub category: i64,

    pub created: String,

    pub day: i64,

    pub description: String,

    pub election_option_id: String,

    pub game_tags: Vec<Option<serde_json::Value>>,

    pub id: String,

    pub metadata: Metadata,

    pub nuts: i64,

    pub phase: i64,

    pub player_tags: Vec<String>,

    pub season: i64,

    pub sim: String,

    pub team_tags: Vec<Option<serde_json::Value>>,

    pub tournament: i64,

    #[serde(rename = "type")]
    pub gammaelectionresult_type: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub after: Option<f64>,

    pub before: Option<f64>,

    pub item_durability: Option<i64>,

    pub item_id: Option<String>,

    pub item_name: Option<String>,

    #[serde(rename = "mod")]
    pub metadata_mod: Option<String>,

    pub mods: Option<Vec<Option<serde_json::Value>>>,

    pub player_item_rating_after: Option<f64>,

    pub player_item_rating_before: Option<i64>,

    pub player_name: Option<String>,

    pub player_rating: Option<f64>,

    pub team_id: Option<String>,

    pub team_name: Option<String>,

    #[serde(rename = "type")]
    pub metadata_type: Option<i64>,
}
