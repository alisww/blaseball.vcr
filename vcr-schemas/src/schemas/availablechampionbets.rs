use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Availablechampionbets {
    inner: Vec<Availablechampionbet>
}

// pub type Availablechampionbets = Vec<Availablechampionbet>;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Availablechampionbet {
    pub division_order: Option<i64>,

    pub emoji: Option<String>,

    pub losses: Option<String>,

    pub main_color: Option<String>,

    pub nickname: Option<String>,

    pub odds: f64,

    pub payout: String,

    pub secondary_color: Option<String>,

    pub sim: Option<String>,

    pub team_batting_rating: Option<f64>,

    pub team_defence_rating: Option<f64>,

    pub team_id: UuidShell,

    pub team_overall_rating: Option<f64>,

    pub team_pitching_rating: Option<f64>,

    pub team_running_rating: Option<f64>,
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub team_won: Option<serde_json::Value>,

    pub wins: Option<String>,
}
