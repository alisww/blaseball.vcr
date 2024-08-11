
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
#[serde(transparent)]
pub struct FuelProgressWrapper {
    inner: Fuelprogress
}

pub type Fuelprogress = Vec<FuelprogressElement>;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct FuelprogressElement {
    pub amount: f64,

    pub id: String,
}
