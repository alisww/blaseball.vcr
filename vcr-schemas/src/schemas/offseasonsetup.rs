use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Offseasonsetup {
    pub blessings: Option<Vec<Blessing>>,
    pub bonuses: Option<Vec<Bonus>>,
    pub decrees: Vec<Decree>,
    pub decrees_to_pass: i64,
    pub gifts: Option<Vec<Gift>>,
    pub wills: Option<Vec<Will>>,
    pub wills_to_pass: Option<i64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Blessing {
    pub description: String,
    pub id: String,
    pub metadata: Option<BlessingMetadata>,
    pub source: Option<i64>,
    pub subheader: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub blessing_type: i64,
    pub value: Option<f64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BlessingMetadata {
    pub amount: Option<Amount>,
    pub amount_max: Option<f64>,
    pub amount_min: Option<f64>,
    pub batch_select: Option<bool>,
    pub blood_type: Option<i64>,
    pub boost_amount: Option<f64>,
    pub boost_stat: Option<i64>,
    pub count: Option<i64>,
    pub count_max: Option<i64>,
    pub count_min: Option<i64>,

    #[serde(rename = "crate")]
    pub metadata_crate: Option<String>,
    pub destination_location: Option<Vec<i64>>,
    pub destination_selection: Option<i64>,
    pub effect: Option<i64>,
    pub event_description: Option<String>,
    pub from_first: Option<bool>,
    pub group_selection: Option<i64>,
    pub group_size: Option<i64>,
    pub group_type: Option<i64>,
    pub impair_amount: Option<f64>,
    pub impair_stat: Option<i64>,
    pub index: Option<i64>,
    pub item: Option<String>,
    pub league_location: Option<i64>,
    pub max: Option<Amount>,
    pub min: Option<Amount>,

    #[serde(rename = "mod")]
    pub metadata_mod: Option<String>,

    pub mods: Option<Vec<String>>,

    pub mod_type: Option<i64>,

    pub percentage: Option<f64>,

    pub player_count: Option<i64>,

    pub player_selection: Option<i64>,

    pub rating: Option<i64>,

    pub reno: Option<String>,

    pub repeat_max: Option<i64>,

    pub repeat_min: Option<i64>,

    pub selection_type: Option<i64>,

    pub sort_type: Option<i64>,

    pub source_location: Option<Vec<i64>>,

    pub source_mods: Option<Vec<String>>,

    pub stat: Option<i64>,

    pub table: Option<String>,

    pub target_location: Option<Vec<i64>>,

    pub target_selection: Option<i64>,

    pub team_selection: Option<i64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    #[serde(rename = "_id")]
    pub id: String,

    pub description: String,

    pub title: String,

    #[serde(rename = "type")]
    pub bonus_type: Option<i64>,

    pub value: Option<f64>,

    pub votes: Option<i64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Decree {
    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub description: String,

    #[serde(rename = "id")]
    pub decree_id: Option<String>,

    pub metadata: Option<DecreeMetadata>,

    pub source: Option<i64>,

    pub title: String,

    #[serde(rename = "type")]
    pub decree_type: i64,

    pub votes: Option<i64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DecreeMetadata {
    pub event_description: String,

    #[serde(rename = "mod")]
    pub metadata_mod: String,

    pub mod_type: i64,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Gift {
    pub description: String,

    pub id: String,

    pub metadata: GiftMetadata,

    pub title: String,

    #[serde(rename = "type")]
    pub gift_type: i64,

    pub value: i64,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GiftMetadata {
    pub amount_max: Option<i64>,

    pub amount_min: Option<i64>,

    #[serde(rename = "crate")]
    pub metadata_crate: Option<String>,

    pub destination: Option<Vec<i64>>,

    pub destination_distribution: Option<i64>,

    pub destination_index: Option<i64>,

    pub destination_location: Option<Vec<i64>>,

    pub destination_selection: Option<i64>,

    pub element0: Option<String>,

    pub event_description: Option<String>,

    pub item: Option<String>,

    pub item_selection: Option<i64>,

    #[serde(rename = "mod")]
    pub metadata_mod: Option<String>,

    pub mod_type: Option<i64>,

    pub percent: Option<f64>,

    pub player_selection: Option<i64>,

    #[serde(rename = "playerTarget0")]
    pub player_target0: Option<String>,

    pub repeat_max: Option<i64>,

    pub repeat_min: Option<i64>,

    pub source_location: Option<Vec<i64>>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Will {
    pub description: String,

    #[serde(rename = "id")]
    pub id: String,

    pub info: Vec<Info>,

    pub source: Option<i64>,

    pub title: String,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub description: String,

    pub filters: Filters,

    #[serde(rename = "type")]
    pub info_type: i64,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub equivalence: Option<f64>,

    pub has_mods: Option<bool>,

    pub league_location: Option<String>,

    pub mods: Option<Vec<String>>,

    pub position: Option<String>,

    pub ratings: Option<i64>,
}

#[derive(BorshSerialize, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Amount {
    Double(f64),

    String(String),
}
