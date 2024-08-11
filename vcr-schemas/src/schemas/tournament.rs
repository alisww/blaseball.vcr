use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub description: String,

    pub finals_name: String,

    pub id: String,

    pub index: i64,

    pub name: String,

    pub playoffs: Uuid,

    pub teams: Vec<UuidShell>,
}
