
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Playoffs {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<Uuid>,

    pub bracket: Option<i64>,

    #[serde(rename = "id")]
    pub playoffs_id: Option<Uuid>,

    pub name: String,

    pub number_of_rounds: i64,

    pub playoff_day: i64,

    pub round: Option<i64>,

    pub rounds: Vec<Uuid>,

    pub season: i64,

    pub tomorrow_round: Option<i64>,

    pub tournament: Option<i64>,

    pub winner: Option<Uuid>,
}
