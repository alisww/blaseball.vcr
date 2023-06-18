
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]
#[serde(rename_all = "camelCase")]
pub struct Sponsordata {
    pub hide_header_on_widget: bool,

    pub sponsor_button_text: String,

    pub sponsor_description: String,

    pub sponsor_link: String,

    pub sponsor_name: String,
}
