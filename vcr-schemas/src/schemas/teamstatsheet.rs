use serde::*;
use vhs_diff::*;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Diff, Patch, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Teamstatsheet {
    #[serde(rename = "created")]
    pub created: Option<String>,

    #[serde(rename = "gamesPlayed")]
    pub games_played: i64,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "losses")]
    pub losses: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "playerStats")]
    pub player_stats: Vec<String>,

    #[serde(rename = "seasonId")]
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub season_id: Option<serde_json::Value>,

    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(rename = "wins")]
    pub wins: i64,
}
