
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
pub struct Thebeat {
    pub collection: Vec<Collection>,

    pub recap: Recap,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    pub contents: Contents,

    pub date: String,

    pub id: String,

    pub title: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub articles: Vec<Article>,

    pub blaseball_link: String,

    pub closing: String,

    pub intro: String,

    pub special_headlines: Vec<SpecialHeadline>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Article {
    pub article: String,

    pub heading: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct SpecialHeadline {
    pub color: Option<String>,

    pub heading: Option<String>,

    pub subheading: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Recap {
    pub beat: String,

    pub content: Vec<Content>,

    pub deeper_content: Vec<DeeperContent>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub header: String,

    pub text: Vec<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeeperContent {
    pub header: String,

    pub text: Vec<String>,
}
