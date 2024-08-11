
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

// #[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
// #[repr(transparent)]
// #[serde(transparent)]
// pub struct Gammaelectiondetails {
//     inner:  Vec<Gammaelectiondetail>
// }

// // pub type Gammaelectiondetails = Vec<Gammaelectiondetail>;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Gammaelectiondetails {
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value")]
    inner: serde_json::Value
}