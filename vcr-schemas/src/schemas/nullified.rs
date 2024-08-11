
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Nullified {
    inner: NullifiedInner
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum NullifiedInner {
    NullifiedClass(NullifiedClass),

    StringArray(Vec<String>),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct NullifiedClass {
    pub history: Option<Vec<History>>,

    pub rules: Vec<String>,

    pub size: Option<f64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct History {
    pub day: i64,

    pub season: i64,

    pub size: f64,
}
