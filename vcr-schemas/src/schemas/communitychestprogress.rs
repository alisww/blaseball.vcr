
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CommunityChestProgress {
    pub chests_unlocked: i64,

    pub progress: Progress,

    pub runs: Progress,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Progress {
    Integer(i64),

    String(String),
}
