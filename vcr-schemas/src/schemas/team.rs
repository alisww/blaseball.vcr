use vcr_lookups::UuidShell;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: Option<UuidShell>,

    pub bench: Option<Vec<UuidShell>>,

    pub bullpen: Option<Vec<UuidShell>>,

    pub card: Option<i64>,

    pub championships: i64,

    pub deceased: Option<bool>,

    pub division_id: Option<String>,

    pub e_density: Option<f64>,

    pub emoji: String,

    pub e_velocity: Option<f64>,

    pub evolution: Option<i64>,

    pub full_name: String,

    pub game_attr: Option<Vec<String>>,

    #[serde(rename = "id")]
    pub team_id: Option<UuidShell>,

    pub im_position: Option<ImPosition>,

    pub league_id: Option<String>,

    pub level: Option<i64>,

    pub lineup: Vec<UuidShell>,

    pub location: String,

    pub main_color: String,

    pub nickname: String,

    #[borsh(serialize_with = "crate::serde_json_borsh::serialize_json_value_vecopt")]
    pub permanent_attributes: Option<Vec<serde_json::Value>>,

    pub perm_attr: Option<Vec<String>>,

    pub rotation: Vec<UuidShell>,

    pub rotation_slot: Option<i64>,

    pub seas_attr: Option<Vec<String>>,

    pub season_attributes: Option<Vec<String>>,

    pub season_shames: i64,

    pub season_shamings: i64,

    pub secondary_color: String,

    pub shadows: Option<Vec<UuidShell>>,

    pub shame_runs: f64,

    pub shorthand: String,

    pub slogan: String,

    pub stadium: Option<Uuid>,

    pub state: Option<State>,

    pub subleague_id: Option<String>,

    pub team_spirit: Option<i64>,

    pub total_shames: i64,

    pub total_shamings: i64,

    pub tournament_wins: Option<i64>,

    pub underchampionships: Option<i64>,

    pub week_attr: Option<Vec<String>>,

    pub win_streak: Option<i64>,
}

impl Team {
    pub fn id(&self) -> Uuid {
        self.id.or(self.team_id).unwrap().as_uuid()
    }
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ImPosition {
    Double(f64),

    DoubleArray(Vec<f64>),
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub donated_shame: Option<f64>,

    pub fax_machine: Option<FaxMachine>,

    pub game_mod_sources: Option<GameModSources>,

    #[serde(rename = "halfinning_plays")]
    pub halfinning_plays: Option<i64>,

    #[serde(rename = "imp_motion")]
    pub imp_motion: Option<Vec<ImpMotion>>,

    pub nullified: Option<bool>,

    pub overflow_runs: Option<f64>,

    pub perm_mod_sources: Option<PermModSources>,

    pub redacted: Option<bool>,

    pub scattered: Option<Scattered>,

    pub stolen_players: Option<Vec<StolenPlayer>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FaxMachine {
    pub runs_needed: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GameModSources {
    #[serde(rename = "0")]
    pub the_0: Option<Vec<String>>,

    pub a: Option<Vec<String>>,

    pub aa: Option<Vec<String>>,

    pub aaa: Option<Vec<String>>,

    pub acidic: Option<Vec<String>>,

    pub affinity_for_crows: Option<Vec<String>>,

    pub ambush: Option<Vec<String>>,

    pub base_instincts: Option<Vec<String>>,

    pub bird_seed: Option<Vec<String>>,

    pub blackhole_payouts: Option<Vec<String>>,

    pub bottom_dweller: Option<Vec<String>>,

    pub carcinization: Option<Vec<String>>,

    pub early_to_party: Option<Vec<String>>,

    pub electric: Option<Vec<String>>,

    pub extra_strike: Option<Vec<String>>,

    pub fiery: Option<Vec<String>>,

    pub fireproof: Option<Vec<String>>,

    pub free_gift: Option<Vec<String>>,

    pub good_riddance: Option<Vec<String>>,

    pub growth: Option<Vec<String>>,

    pub h20: Option<Vec<String>>,

    pub heavy_handed: Option<Vec<String>>,

    pub high_pressure: Option<Vec<String>>,

    pub home_field: Option<Vec<String>>,

    pub late_to_party: Option<Vec<String>>,

    pub life_of_party: Option<Vec<String>>,

    pub light_handed: Option<Vec<String>>,

    pub love: Option<Vec<String>>,

    pub maintenance_mode: Option<Vec<String>>,

    pub middling: Option<Vec<String>>,

    pub moderation: Option<Vec<String>>,

    pub o_no: Option<Vec<String>>,

    pub overperforming: Option<Vec<String>>,

    pub party_time: Option<Vec<String>>,

    pub popcorn_payouts: Option<Vec<String>>,

    pub psychic: Option<Vec<String>>,

    pub sealant: Option<Vec<String>>,

    pub shame_giver: Option<Vec<String>>,

    pub sinking_ship: Option<Vec<String>>,

    pub stalepopcorn_payouts: Option<Vec<String>>,

    pub sun2_payouts: Option<Vec<String>>,

    pub sun_kissed: Option<Vec<String>>,

    pub traveling: Option<Vec<String>>,

    pub underperforming: Option<Vec<String>>,

    pub undersea: Option<Vec<String>>,

    pub unholey: Option<Vec<String>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ImpMotion {
    pub day: i64,

    pub im_position: Vec<f64>,

    pub season: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PermModSources {
    pub overperforming: Option<Vec<String>>,

    pub underperforming: Option<Vec<String>>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Scattered {
    pub full_name: String,

    pub location: String,

    pub nickname: String,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StolenPlayer {
    pub id: String,

    pub victim_index: i64,

    pub victim_location: i64,

    pub victim_team_id: String,
}
