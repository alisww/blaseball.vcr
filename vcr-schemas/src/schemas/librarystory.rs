use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct LibrarystoryWrapper {
    inner: Vec<LibrarystoryElement>,
}

#[derive(BorshSerialize, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LibrarystoryElement {
    pub category: i64,
    pub created: String,
    pub day: i64,
    pub description: String,
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_vecopt")]
    pub game_tags: Option<Vec<serde_json::Value>>,
    pub id: Uuid,
    pub metadata: Metadata,
    pub nuts: i64,
    pub phase: i64,
    pub player_tags: Option<Vec<Uuid>>,
    pub season: i64,
    pub team_tags: Option<Vec<Uuid>>,
    pub tournament: i64,
    #[serde(rename = "type")]
    pub librarystory_type: i64,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub add_location: Option<i64>,
    pub add_player_id: Option<String>,
    pub add_player_name: Option<String>,
    pub after: Option<f64>,
    pub amount: Option<i64>,
    pub away: Option<String>,
    pub before: Option<f64>,
    pub being: Option<Being>,
    pub beings: Option<Vec<String>>,
    pub coins_after: Option<i64>,
    pub coins_before: Option<i64>,
    pub current: Option<f64>,
    pub data_votes: Option<i64>,
    pub from: Option<String>,
    pub history: Option<bool>,
    pub home: Option<String>,
    pub id: Option<String>,
    pub in_player_id: Option<String>,
    pub in_player_name: Option<String>,
    pub in_team_id: Option<String>,
    pub in_team_name: Option<String>,
    pub item_durability: Option<i64>,
    pub item_health_after: Option<i64>,
    pub item_health_before: Option<i64>,
    pub item_id: Option<String>,
    pub item_name: Option<String>,
    pub item_received_name: Option<String>,
    pub item_traded_name: Option<String>,
    pub lines: Option<Vec<String>>,
    pub location: Option<i64>,
    pub maximum: Option<i64>,
    #[serde(rename = "mod")]
    pub metadata_mod: Option<String>,
    pub mods: Option<Vec<String>>,
    pub mods_gained: Option<Vec<String>>,
    pub mods_lost: Option<Vec<String>>,
    pub out_player_id: Option<String>,
    pub out_player_name: Option<String>,
    pub out_team_id: Option<String>,
    pub out_team_name: Option<String>,

    pub place: Option<String>,

    pub player_id: Option<String>,

    pub player_item_rating_after: Option<f64>,

    pub player_item_rating_before: Option<f64>,

    pub player_name: Option<String>,

    pub player_rating: Option<f64>,

    pub recharge: Option<i64>,

    pub redacted: Option<bool>,

    pub retreat_location: Option<i64>,

    pub retreat_player_id: Option<String>,

    pub retreat_player_name: Option<String>,

    pub scales: Option<i64>,

    pub team_id: Option<String>,

    pub team_name: Option<String>,

    pub title: Option<String>,

    pub to: Option<String>,

    pub total_votes: Option<i64>,

    #[serde(rename = "type")]
    pub metadata_type: Option<i64>,

    pub votes: Option<i64>,

    pub weather: Option<Being>,

    pub will_votes: Option<i64>,

    pub winner: Option<String>,
}

#[derive(BorshSerialize, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Being {
    Integer(i64),

    String(String),
}
