use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(alias = "_id")]
    pub id: UuidShell,

    pub a_baserunning_rating: Option<i64>,

    pub a_defense_rating: Option<i64>,

    pub a_hitting_rating: Option<i64>,

    pub anticapitalism: f64,

    pub a_pitching_rating: Option<i64>,

    pub armor: Option<String>,

    pub baserunning_rating: Option<f64>,

    pub base_thirst: f64,

    pub bat: Option<String>,

    pub blood: Option<i64>,

    pub buoyancy: f64,

    pub chasiness: f64,

    pub cinnamon: Option<f64>,

    pub coffee: Option<i64>,

    pub coldness: f64,

    pub consecutive_hits: Option<i64>,

    pub continuation: f64,

    pub deceased: Option<bool>,

    pub defense_rating: Option<f64>,

    pub divinity: f64,

    pub e_density: Option<f64>,

    pub evolution: Option<i64>,

    pub fate: Option<i64>,

    pub game_attr: Option<Vec<String>>,

    pub ground_friction: f64,

    pub hit_streak: Option<i64>,

    pub hitting_rating: Option<f64>,

    pub indulgence: f64,

    pub item_attr: Option<Vec<String>>,

    pub items: Option<Vec<ItemElement>>,

    pub laserlikeness: f64,

    pub league_team_id: Option<UuidShell>,

    pub martyrdom: f64,

    pub moxie: f64,

    pub musclitude: f64,

    pub name: String,

    pub omniscience: f64,

    pub overpowerment: f64,

    pub patheticism: f64,

    pub peanut_allergy: Option<bool>,

    pub perm_attr: Option<Vec<String>>,

    pub pitching_rating: Option<f64>,

    pub pressurization: f64,

    pub ritual: Option<String>,

    pub ruthlessness: f64,

    pub seas_attr: Option<Vec<String>>,

    pub shakespearianism: f64,

    pub soul: i64,

    pub spin: Option<i64>,

    pub state: Option<PlayerState>,

    pub suppression: f64,

    pub tenaciousness: f64,

    pub thwackability: f64,

    pub total_fingers: i64,

    pub tournament_team_id: Option<UuidShell>,

    pub tragicness: f64,

    pub unthwackability: f64,

    pub watchfulness: f64,

    pub week_attr: Option<Vec<String>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ItemElement {
    ItemClass(ItemClass),

    String(String),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ItemClass {
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

    pub state: Option<ItemState>,

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
pub struct ItemState {
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

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub cut_this_election: Option<bool>,

    pub elsewhere: Option<Elsewhere>,

    pub game_mod_sources: Option<GameModSources>,

    pub hunches: Option<Vec<i64>>,

    pub investigations: Option<i64>,

    pub item_mod_sources: Option<ItemModSources>,

    pub necromancied_this_election: Option<bool>,

    pub original: Option<String>,

    pub perm_mod_sources: Option<PermModSources>,

    pub pre_investigation_index: Option<i64>,

    pub pre_investigation_location: Option<i64>,

    pub pre_investigation_team: Option<String>,

    pub redacted: Option<bool>,

    pub seas_mod_sources: Option<SeasModSources>,

    pub unscattered_name: Option<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Elsewhere {
    pub day: i64,

    pub season: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GameModSources {
    pub overperforming: Vec<String>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ItemModSources {
    pub ambitious: Option<Vec<String>>,

    pub bird_seed: Option<Vec<String>>,

    pub blaserunning: Option<Vec<String>>,

    pub careful: Option<Vec<String>>,

    pub chunky: Option<Vec<String>>,

    pub coasting: Option<Vec<String>>,

    pub coffee_peril: Option<Vec<String>>,

    pub containment: Option<Vec<String>>,

    pub curse_of_crows: Option<Vec<String>>,

    pub double_payouts: Option<Vec<String>>,

    pub entangled: Option<Vec<String>>,

    pub extra_strike: Option<Vec<String>>,

    pub fire_eater: Option<Vec<String>>,

    pub fire_protector: Option<Vec<String>>,

    pub fireproof: Option<Vec<String>>,

    pub flickering: Option<Vec<String>>,

    pub fliickerrriiing: Option<Vec<String>>,

    pub force: Option<Vec<String>>,

    pub gravity: Option<Vec<String>>,

    pub haunted: Option<Vec<String>>,

    pub high_pressure: Option<Vec<String>>,

    pub maximalist: Option<Vec<String>>,

    pub minimized: Option<Vec<String>>,

    pub night_vision: Option<Vec<String>>,

    pub offworld: Option<Vec<String>>,

    pub parasite: Option<Vec<String>>,

    pub pro_skater: Option<Vec<String>>,

    pub repeating: Option<Vec<String>>,

    pub reverberating: Option<Vec<String>>,

    pub slow_build: Option<Vec<String>>,

    pub smooth: Option<Vec<String>>,

    pub soundproof: Option<Vec<String>>,

    pub spicy: Option<Vec<String>>,

    pub squiddish: Option<Vec<String>>,

    pub steeled: Option<Vec<String>>,

    pub subtractor: Option<Vec<String>>,

    pub superwanderer: Option<Vec<String>>,

    pub trader: Option<Vec<String>>,

    pub traitor: Option<Vec<String>>,

    pub traveling: Option<Vec<String>>,

    pub unambitious: Option<Vec<String>>,

    pub uncertain: Option<Vec<String>>,

    pub underhanded: Option<Vec<String>>,

    pub unfreezable: Option<Vec<String>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PermModSources {
    pub affinity_for_crows: Option<Vec<String>>,

    pub alternate: Option<Vec<String>>,

    pub attractor: Option<Vec<String>>,

    pub careful: Option<Vec<String>>,

    pub chunky: Option<Vec<String>>,

    pub cluttered: Option<Vec<String>>,

    pub credit_to_the_team: Option<Vec<String>>,

    pub debt_three: Option<Vec<String>>,

    pub double_payouts: Option<Vec<String>>,

    pub ego1: Option<Vec<String>>,

    pub ego2: Option<Vec<String>>,

    pub ego3: Option<Vec<String>>,

    pub ego4: Option<Vec<String>>,

    pub elsewhere: Option<Vec<String>>,

    pub fire_eater: Option<Vec<String>>,

    pub flinch: Option<Vec<String>>,

    pub friend_of_crows: Option<Vec<String>>,

    pub gaudy: Option<Vec<String>>,

    pub hard_boiled: Option<Vec<String>>,

    pub homebody: Option<Vec<String>>,

    pub honey_roasted: Option<Vec<String>>,

    pub magmatic: Option<Vec<String>>,

    pub minimalist: Option<Vec<String>>,

    pub negative: Option<Vec<String>>,

    pub non_idolized: Option<Vec<String>>,

    pub overperforming: Option<Vec<String>>,

    pub perk: Option<Vec<String>>,

    pub returned: Option<Vec<String>>,

    pub reverberating: Option<Vec<String>>,

    pub scattered: Option<Vec<String>>,

    pub scrambled: Option<Vec<String>>,

    pub seeker: Option<Vec<String>>,

    pub shelled: Option<Vec<String>>,

    pub siphon: Option<Vec<String>>,

    pub skipping: Option<Vec<String>>,

    pub smooth: Option<Vec<String>>,

    pub spicy: Option<Vec<String>>,

    pub squiddish: Option<Vec<String>>,

    pub superallergic: Option<Vec<String>>,

    pub superyummy: Option<Vec<String>>,

    pub swim_bladder: Option<Vec<String>>,

    pub triple_threat: Option<Vec<String>>,

    pub uncertain: Option<Vec<String>>,

    pub undefined: Option<Vec<String>>,

    pub underperforming: Option<Vec<String>>,

    pub undertaker: Option<Vec<String>>,

    pub walk_in_the_park: Option<Vec<String>>,

    pub wanderer: Option<Vec<String>>,

    pub wild: Option<Vec<String>>,

    pub yolked: Option<Vec<String>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SeasModSources {
    pub alternate: Option<Vec<String>>,

    pub chunky: Option<Vec<String>>,

    pub debt_three: Option<Vec<String>>,

    pub ego2: Option<Vec<String>>,

    pub elsewhere: Option<Vec<String>>,

    pub fire_eater: Option<Vec<String>>,

    pub friend_of_crows: Option<Vec<String>>,

    pub honey_roasted: Option<Vec<String>>,

    pub minimalist: Option<Vec<String>>,

    pub overperforming: Option<Vec<String>>,

    pub perk: Option<Vec<String>>,

    pub returned: Option<Vec<String>>,

    pub scattered: Option<Vec<String>>,

    pub shelled: Option<Vec<String>>,

    pub siphon: Option<Vec<String>>,

    pub smooth: Option<Vec<String>>,

    pub spicy: Option<Vec<String>>,

    pub superallergic: Option<Vec<String>>,

    pub superyummy: Option<Vec<String>>,

    pub swim_bladder: Option<Vec<String>>,

    pub triple_threat: Option<Vec<String>>,

    pub underperforming: Option<Vec<String>>,

    pub wanderer: Option<Vec<String>>,
}
