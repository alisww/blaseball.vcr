
use borsh::BorshSerialize;
use serde::{Serialize, Deserialize};

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    pub collection: Vec<Collection>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub background: String,

    pub color: String,

    pub description: String,

    pub descriptions: Option<Descriptions>,

    pub id: String,

    pub text_color: String,

    pub title: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Descriptions {
    pub ballpark: Option<String>,

    pub league: Option<String>,

    pub player: Option<String>,

    pub team: Option<String>,
}
