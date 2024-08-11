
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct League {
    pub id: Uuid,

    pub name: String,

    pub subleagues: Vec<Uuid>,

    pub tiebreakers: Uuid,
}
