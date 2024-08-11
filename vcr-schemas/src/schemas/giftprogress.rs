use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use vhs_diff::{Diff, Patch};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Diff, Patch, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Giftprogress {
    pub team_progress: HashMap<Uuid, TeamProgress>,
    pub team_wish_lists: HashMap<Uuid, Vec<WishlistProgress>>,
}

#[derive(BorshSerialize, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TeamProgress {
    total: i64,
    to_next: f64,
}

#[derive(BorshSerialize, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct WishlistProgress {
    bonus: String,
    percent: f64,
}
