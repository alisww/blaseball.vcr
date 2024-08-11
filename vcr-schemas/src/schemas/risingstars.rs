use serde::{Deserialize, Serialize};
use vhs_diff::{Diff, Patch};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Risingstars {
    #[serde(rename = "stars")]
    pub stars: Vec<String>,
}
