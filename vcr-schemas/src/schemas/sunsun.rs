
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct Sunsun {
    pub current: f64,

    pub maximum: i64,

    #[serde(default)]
    pub recharge: Option<i64>,
}
