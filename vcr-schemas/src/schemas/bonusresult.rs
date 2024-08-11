use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Bonusresult {
    pub bonus_id: String,

    pub bonus_title: String,

    pub description: String,

    pub highest_team: String,

    pub highest_team_votes: i64,

    pub id: String,

    pub team_id: UuidShell,

    pub team_votes: i64,

    pub total_votes: i64,
}
