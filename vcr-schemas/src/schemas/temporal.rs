
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct Temporal {
    pub alpha: Option<i64>,

    pub beta: Option<i64>,

    pub delta: Option<bool>,

    pub doc: Option<Doc>,

    pub epsilon: Option<bool>,

    pub eta: Option<i64>,

    pub gamma: Option<i64>,

    pub id: Option<String>,

    pub iota: Option<i64>,

    pub theta: Option<String>,

    pub zeta: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Doc {
    pub alpha: i64,

    pub beta: i64,

    pub delta: bool,

    pub epsilon: bool,

    pub eta: Option<i64>,

    pub gamma: i64,

    pub id: String,

    pub iota: Option<i64>,

    pub theta: Option<String>,

    pub zeta: String,
}
