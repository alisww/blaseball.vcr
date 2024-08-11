
use std::collections::HashMap;
use borsh::BorshSerialize;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use vcr_lookups::UuidShell;

#[derive(BorshSerialize, Deserialize, Serialize, Copy, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum FloatOrI64 {
    F64(f64),
    I64(i64)
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub games_played: Option<HashMap<UuidShell, Option<FloatOrI64>>>,

    #[serde(rename = "id")]
    pub standings_id: Option<String>,

    pub losses: Option<HashMap<UuidShell, Option<FloatOrI64>>>,

    pub runs: Option<HashMap<UuidShell, Option<FloatOrI64>>>,
    pub wins: HashMap<UuidShell, Option<FloatOrI64>>
}

impl Standings {
    pub fn id(&self) -> Uuid {
        self.id.as_ref().or(self.standings_id.as_ref()).and_then(|v| v.parse::<Uuid>().ok()).unwrap()
    }
}