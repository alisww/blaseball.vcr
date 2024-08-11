
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GlobaleventsWrapper {
    inner: Globalevents
}

pub type Globalevents = Vec<Globalevent>;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct Globalevent {
    #[serde(rename = "__v")]
    pub v: Option<i64>,

    #[serde(rename = "_id")]
    pub id: Option<String>,

    pub expire: Option<String>,

    #[serde(rename = "id")]
    pub globalevent_id: Option<String>,

    pub msg: String,
}
