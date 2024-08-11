
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub old_id: Option<Uuid>,
    pub id: Option<Uuid>,
    
    pub league: Uuid,

    pub rules: Uuid,

    pub schedule: Option<Uuid>,

    pub season_number: i64,

    pub standings: Uuid,

    pub stats: Uuid,

    pub terminology: Uuid,

    pub total_days_in_season: Option<i64>,
}

impl Season {
    pub fn id(&self) -> Option<Uuid> {
        self.id.or(self.old_id)
    }
}