
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Sim {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub agitations: Option<i64>,

    pub attr: Option<Vec<String>>,

    pub day: i64,

    pub do_the_thing: Option<bool>,

    pub earlpostseason_date: Option<String>,

    pub earlseason_date: Option<String>,

    pub earlsiesta_date: Option<String>,

    pub eat_the_rich: Option<bool>,

    pub election_date: Option<String>,

    pub endseason_date: Option<String>,

    pub era_color: Option<String>,

    pub era_title: String,

    pub gods_day_date: Option<String>,

    #[serde(rename = "id")]
    pub sim_id: Option<String>,

    pub labour_one: Option<i64>,

    pub latepostseason_date: Option<String>,

    pub lateseason_date: Option<String>,

    pub latesiesta_date: Option<String>,

    pub league: String,

    pub menu: Option<String>,

    pub midseason_date: Option<String>,

    pub next_election_end: Option<String>,

    pub next_phase_time: String,

    pub next_season_start: Option<String>,

    pub opened_book: Option<bool>,

    pub phase: i64,

    pub play_off_round: Option<i64>,

    pub playoffs: Option<Playoffs>,

    pub preseason_date: Option<String>,

    pub rules: String,

    pub salutations: Option<i64>,

    pub season: i64,

    pub season_id: Option<String>,

    pub sim_end: Option<String>,

    pub sim_start: Option<String>,

    pub state: Option<State>,

    pub sub_era_color: Option<String>,

    pub sub_era_title: Option<String>,

    pub terminology: String,

    pub tournament: Option<i64>,

    pub tournament_round: Option<i64>,

    pub twgo: Option<String>,

    pub unlocked_interviews: Option<bool>,

    pub unlocked_peanuts: Option<bool>,
}


#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Playoffs {
    String(Uuid),

    StringArray(Vec<Uuid>),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    #[serde(rename = "phaseOnHold")]
    pub phase_on_hold: Option<i64>,

    pub scheduled_game_event: Option<String>,
}
