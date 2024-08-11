
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub baserunning_rating: Option<f64>,

    pub defense_rating: Option<f64>,

    pub durability: i64,
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub forger: Option<serde_json::Value>,
    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_opt")]
    pub forger_name: Option<serde_json::Value>,

    pub health: i64,

    pub hitting_rating: Option<f64>,

    pub id: String,

    pub name: String,

    pub pitching_rating: Option<f64>,

    pub post_prefix: Option<PostPrefix>,

    pub prefixes: Option<Vec<Prefix>>,

    pub pre_prefix: Option<PrePrefix>,

    pub root: Root,

    pub state: Option<State>,

    pub suffix: Option<Suffix>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostPrefix {
    pub adjustments: Vec<PostPrefixAdjustment>,

    pub name: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostPrefixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: Option<f64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PrePrefix {
    pub adjustments: Vec<PrePrefixAdjustment>,

    pub name: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PrePrefixAdjustment {
    pub stat: i64,

    #[serde(rename = "type")]
    pub adjustment_type: i64,

    pub value: f64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Prefix {
    pub adjustments: Vec<PrefixAdjustment>,

    pub name: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PrefixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: Option<i64>,

    pub value: Option<f64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub adjustments: Vec<RootAdjustment>,

    pub name: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct RootAdjustment {
    pub stat: i64,

    #[serde(rename = "type")]
    pub adjustment_type: Option<i64>,

    pub value: f64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub original: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Suffix {
    pub adjustments: Vec<SuffixAdjustment>,

    pub name: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct SuffixAdjustment {
    #[serde(rename = "mod")]
    pub adjustment_mod: Option<String>,

    pub stat: Option<i64>,

    #[serde(rename = "type")]
    pub adjustment_type: Option<i64>,

    pub value: Option<f64>,
}
