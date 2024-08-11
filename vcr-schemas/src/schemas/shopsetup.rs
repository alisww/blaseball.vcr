
use serde::{Serialize, Deserialize};
use borsh::BorshSerialize;

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Shopsetup {
    pub menu: Vec<String>,

    pub snack_data: SnackData,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SnackData {
    pub black_hole_tiers: Vec<BlackHoleTier>,

    pub consumer_tiers: Vec<ConsumerTier>,

    pub flood_clear_tiers: Vec<FloodClearTier>,

    pub idol_hits_tiers: Vec<IdolHitsTier>,

    pub idol_homer_allowed_tiers: Vec<IdolHomerAllowedTier>,

    pub idol_homers_tiers: Vec<IdolHomersTier>,

    pub idol_pitcher_lose_tiers: Vec<IdolPitcherLoseTier>,

    pub idol_pitcher_win_tiers: Vec<IdolPitcherWinTier>,

    pub idol_shutouts_tiers: Vec<IdolShutoutsTier>,

    pub idol_steal_tiers: Vec<IdolStealTier>,

    pub idol_strikeouts_tiers: Vec<IdolStrikeoutsTier>,

    pub incineration_tiers: Vec<IncinerationTier>,

    pub max_bet_tiers: Vec<MaxBetTier>,

    pub sun_two_tiers: Vec<SunTwoTier>,

    pub team_loss_coin_tiers: Vec<TeamLossCoinTier>,

    pub team_shamed_tiers: Vec<TeamShamedTier>,

    pub team_shaming_tiers: Vec<TeamShamingTier>,

    pub team_win_coin_tiers: Vec<TeamWinCoinTier>,

    pub time_off_tiers: Vec<TimeOffTier>,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlackHoleTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConsumerTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FloodClearTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolHitsTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolHomerAllowedTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolHomersTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolPitcherLoseTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolPitcherWinTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolShutoutsTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolStealTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IdolStrikeoutsTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncinerationTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct MaxBetTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct SunTwoTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamLossCoinTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamShamedTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamShamingTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamWinCoinTier {
    pub amount: i64,

    pub price: i64,
}

#[derive(BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeOffTier {
    pub amount: i64,

    pub price: i64,
}
