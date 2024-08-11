use serde::*;
use vhs_diff::*;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Diff, Patch, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Seasonstatsheet {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "teamStats")]
    pub team_stats: Vec<String>,
}
