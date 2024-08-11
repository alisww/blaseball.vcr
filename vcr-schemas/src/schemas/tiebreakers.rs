
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Tiebreakers {
    inner: TiebreakersInner
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum TiebreakersInner {
    TiebreakerArray(Vec<Tiebreaker>),

    TiebreakersClass(TiebreakersClass),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tiebreaker {
    pub id: Uuid,

    pub order: Vec<Uuid>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TiebreakersClass {
    pub id: Uuid,

    pub order: Vec<Uuid>,
}
