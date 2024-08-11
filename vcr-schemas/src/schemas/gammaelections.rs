
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Gammaelections {
    inner: Vec<Gammaelection>
}
// pub type Gammaelections = Vec<Gammaelection>;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Gammaelection {
    pub choice_type: String,

    pub description: String,

    pub election_complete: bool,

    pub end_date: String,

    pub icon: String,

    pub id: String,
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub maximum_allowed: Option<serde_json::Value>,

    pub name: String,

    pub start_date: String,
}
