
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Idols {
    inner: IdolsInner
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum IdolsInner {
    IdolArray(Vec<Idol>),

    IdolsClass(IdolsClass),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Idol {
    pub id: Option<String>,

    pub player_id: String,

    pub total: Option<i64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolsClass {
    pub data: Data,

    pub idols: Vec<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub strictly_confidential: i64,
}
