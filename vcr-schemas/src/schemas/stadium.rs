
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Stadium {
    pub birds: i64,

    pub elongation: f64,

    pub filthiness: f64,

    pub fortification: f64,

    pub forwardness: f64,

    pub grandiosity: f64,

    pub hype: f64,

    pub id: String,

    pub inconvenience: f64,

    pub luxuriousness: f64,

    pub main_color: String,

    pub model: Option<i64>,

    pub mods: Vec<String>,

    pub mysticism: f64,

    pub name: String,

    pub nickname: String,

    pub obtuseness: f64,

    pub ominousness: f64,

    pub reno_cost: i64,

    pub reno_discard: Vec<String>,

    pub reno_hand: Vec<String>,

    pub reno_log: RenoLog,

    pub secondary_color: String,

    pub state: State,

    pub team_id: String,

    pub tertiary_color: String,

    pub viscosity: f64,

    pub weather: Weather,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct RenoLog {
    pub air_balloons_mod: Option<i64>,

    pub anti_flood_pumps_mod: Option<i64>,

    pub anti_graphene_mod: Option<i64>,

    pub big_bucket_mod: Option<i64>,

    pub bird_hotel_mod: Option<i64>,

    pub birdhouses_mod: Option<i64>,

    pub climate_control_mod: Option<i64>,

    pub condensed_floor_plan_mod: Option<i64>,

    pub cycling_mod: Option<i64>,

    pub echo_chamber_mod: Option<i64>,

    pub event_horizon_mod: Option<i64>,

    pub fax_machine_mod: Option<i64>,

    pub fire_insurance_mod: Option<i64>,

    pub flood_balloons_mod: Option<i64>,

    pub flood_pumps_mod: Option<i64>,

    pub graphene_mod: Option<i64>,

    pub grind_rail_mod: Option<i64>,

    pub hoops_mod: Option<i64>,

    pub hot_air_balloons_mod: Option<i64>,

    pub hotel_motel_mod: Option<i64>,

    pub light_switch_toggle: Option<i64>,

    pub open_floor_plan_mod: Option<i64>,

    pub peanut_mister_mod: Option<i64>,

    pub psychoacoustics_mod: Option<i64>,

    pub salmon_cannons_mod: Option<i64>,

    pub secret_base_mod: Option<i64>,

    pub secret_tunnels_mod: Option<i64>,

    pub solar_panels_mod: Option<i64>,

    pub soundsystem_mod: Option<i64>,

    pub stables_mod: Option<i64>,

    pub sweetener_mod: Option<i64>,

    #[serde(rename = "sweeupdate stadiums seon_cannons_mod")]
    pub sweeupdate_stadiums_seon_cannons_mod: Option<i64>,

    pub thieves_guild_mod: Option<i64>,

    pub turntables_mod: Option<i64>,

    pub very_foul_balls_mod: Option<i64>,

    pub voicemail_mod: Option<i64>,

    pub weather_reports_mod: Option<i64>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(rename = "air_balloons")]
    pub air_balloons: Option<i64>,

    pub event_horizon: Option<bool>,

    pub fax_machine: Option<FaxMachine>,

    #[serde(rename = "flood_balloons")]
    pub flood_balloons: Option<i64>,

    pub solar_panels: Option<bool>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FaxMachine {
    pub runs_needed: f64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Weather {
    #[serde(rename = "1")]
    pub the_1: Option<i64>,

    #[serde(rename = "10")]
    pub the_10: Option<i64>,

    #[serde(rename = "11")]
    pub the_11: Option<i64>,

    #[serde(rename = "13")]
    pub the_13: Option<i64>,

    #[serde(rename = "14")]
    pub the_14: Option<i64>,

    #[serde(rename = "15")]
    pub the_15: Option<i64>,

    #[serde(rename = "16")]
    pub the_16: Option<i64>,

    #[serde(rename = "17")]
    pub the_17: Option<i64>,

    #[serde(rename = "18")]
    pub the_18: Option<i64>,

    #[serde(rename = "19")]
    pub the_19: Option<i64>,

    #[serde(rename = "29")]
    pub the_29: Option<i64>,

    #[serde(rename = "7")]
    pub the_7: Option<i64>,

    #[serde(rename = "8")]
    pub the_8: Option<i64>,

    #[serde(rename = "9")]
    pub the_9: Option<i64>,
}
