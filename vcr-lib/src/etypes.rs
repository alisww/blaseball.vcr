pub use vcr_schemas::bonusresult::Bonusresult;
pub use vcr_schemas::bossfight::Bossfight;
pub use vcr_schemas::communitychestprogress::CommunityChestProgress;
pub use vcr_schemas::decreeresult::Decreeresult;
pub use vcr_schemas::division::Division;
pub use vcr_schemas::eventresult::Eventresult;
pub use vcr_schemas::fuelprogress::FuelProgressWrapper;
pub use vcr_schemas::game::GameUpdate;
pub use vcr_schemas::giftprogress::Giftprogress;
pub use vcr_schemas::globalevents::GlobaleventsWrapper;
pub use vcr_schemas::idols::Idols;
pub use vcr_schemas::item::Item;
pub use vcr_schemas::league::League;
pub use vcr_schemas::librarystory::LibrarystoryWrapper;
pub use vcr_schemas::nullified::Nullified;
pub use vcr_schemas::offseasonrecap::Offseasonrecap;
pub use vcr_schemas::offseasonsetup::Offseasonsetup;
pub use vcr_schemas::player::Player;
pub use vcr_schemas::playoffmatchup::Playoffmatchup;
pub use vcr_schemas::playoffround::Playoffround;
pub use vcr_schemas::playoffs::Playoffs;
pub use vcr_schemas::renovationprogress::Renovationprogress;
pub use vcr_schemas::risingstars::Risingstars;
pub use vcr_schemas::season::Season;
pub use vcr_schemas::shopsetup::Shopsetup;
pub use vcr_schemas::sim::Sim;
pub use vcr_schemas::stadium::Stadium;
pub use vcr_schemas::standings::Standings;
pub use vcr_schemas::subleague::Subleague;
pub use vcr_schemas::sunsun::Sunsun;
pub use vcr_schemas::team::Team;
pub use vcr_schemas::teamelectionstats::Teamelectionstats;
pub use vcr_schemas::temporal::Temporal;
pub use vcr_schemas::tiebreakers::Tiebreakers;
pub use vcr_schemas::tournament::Tournament;
pub use vcr_schemas::tributes::*;
pub use vcr_schemas::vault::Vault;

pub use vcr_schemas::attributes::*;
pub use vcr_schemas::availablechampionbets::*;
pub use vcr_schemas::championcallout::*;
pub use vcr_schemas::dayssincelastincineration::*;
pub use vcr_schemas::fanart::*;
pub use vcr_schemas::feedseasonlist::*;
pub use vcr_schemas::gamestatsheet::*;
pub use vcr_schemas::gammabracket::*;
pub use vcr_schemas::gammaelection::*;
pub use vcr_schemas::gammaelectiondetails::*;
pub use vcr_schemas::gammaelectionresults::*;
pub use vcr_schemas::gammaelections::Gammaelections;
pub use vcr_schemas::gammasim::*;
pub use vcr_schemas::glossarywords::*;
pub use vcr_schemas::peanutpower::*;
pub use vcr_schemas::playerstatsheet::*;
pub use vcr_schemas::seasonstatsheet::*;
pub use vcr_schemas::sponsordata::*;
pub use vcr_schemas::stadiumprefabs::*;
pub use vcr_schemas::teamstatsheet::*;
pub use vcr_schemas::thebeat::*;
pub use vcr_schemas::thebook::*;

use crate::stream_data::thisidisstaticyo;

use serde::ser::{Serialize, Serializer};
use std::str::FromStr;

use crate::stream_data::db::StreamEntityWrapper;
use borsh::{BorshDeserialize, BorshSerialize};
use strum::FromRepr;

#[macro_export]
macro_rules! etypes {
    // "player" -> Player(Player)
    // "fuelprogress" -> FuelProgress(FuelProgressWrapper)
    ($($name:literal -> $variant:ident ($what:ty) ),*) => {
        pub enum DynamicEntity {
            $(
                $variant($what),
            )*
        }

        #[repr(u8)]
        #[derive(BorshSerialize, Clone, Copy, BorshDeserialize, PartialEq, Debug, PartialOrd, Eq, FromRepr)]
        pub enum DynamicEntityType {
            $(
                $variant,
            )*
        }

        $(
            impl From<$what> for DynamicEntity {
                fn from(t: $what) -> Self {
                    DynamicEntity::$variant(t)
                }
            }
        )*

        impl Serialize for DynamicEntity {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
                match self {
                    $(
                        DynamicEntity::$variant(data) => <$what as serde::Serialize>::serialize(data, serializer),
                    )*
                }
            }
        }

        impl FromStr for DynamicEntityType {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    $(
                        $name => Ok(DynamicEntityType::$variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}

etypes! {
    "gameupdate" -> GameUpdate(GameUpdate),
    "bossfight" -> Bossfight(Bossfight),
    "communitychestprogress" -> CommunityChestProgress(CommunityChestProgress),
    "division" -> Division(Division),
    "league" -> League(League),
    "playoffmatchup" -> Playoffmatchup(Playoffmatchup),
    "playoffround" -> Playoffround(Playoffround),
    "playoffs" -> Playoffs(Playoffs),
    "season" -> Season(Season),
    "sim" -> Sim(Sim),
    "stadium" -> Stadium(Stadium),
    "standings" -> Standings(Standings),
    "subleague" -> Subleague(Subleague),
    "team" -> Team(Team),
    "sunsun" -> Sunsun(Sunsun),
    "temporal" -> Temporal(Temporal),
    "tiebreakers" -> Tiebreakers(Tiebreakers),
    "tournament" -> Tournament(Tournament),
    "bonusresult" -> Bonusresult(Bonusresult),
    "decreeresult" -> Decreeresult(Decreeresult),
    "eventresult" -> Eventresult(Eventresult),
    "fuelprogress" -> FuelProgress(FuelProgressWrapper),
    "giftprogress" -> Giftprogress(Giftprogress),
    "globalevents" -> GlobalEvents(GlobaleventsWrapper),
    "idols" -> Idols(Idols),
    "item" -> Item(Item),
    "librarystory" -> LibraryStory(LibrarystoryWrapper),
    "nullified" -> Nullified(Nullified),
    "offseasonrecap" -> Offseasonrecap(Offseasonrecap),
    "offseasonsetup" -> Offseasonsetup(Offseasonsetup),
    "player" -> Player(Player),
    "renovationprogress" -> RenovationProgress(Renovationprogress),
    "risingstars" -> RisingStars(Risingstars),
    "shopsetup" -> ShopSetup(Shopsetup),
    "teamelectionstats" -> TeamElectionStats(Teamelectionstats),
    "vault" -> Vault(Vault),
    "stadiumprefabs" -> StadiumPrefabs(Stadiumprefabs),
    "thebook" -> TheBook(Thebook),
    "thebeat" -> TheBeat(Thebeat),
    "teamstatsheet" -> TeamStatSheet(Teamstatsheet),
    "glossarywords" -> GlossaryWords(Glossarywords),
    "peanutpower" -> PeanutPower(Peanutpower),
    "gammasim" -> GammaSim(Gammasim),
    "gammaelections" -> GammaElections(Gammaelections),
    "gammaelectionresults" -> GammaElectionResults(Gammaelectionresults),
    "gammaelectiondetails" -> GammaElectionDetail(Gammaelectiondetails),
    "gammaelection" -> GammaElection(Gammaelection),
    "gammabracket" -> Gammabracket(Gammabracket),
    "gamestatsheet" -> GameStatSheet(Gamestatsheet),
    "feedseasonlist" -> FeedSeasonList(Feedseasonlist),
    "fanart" -> Fanart(Fanart),
    "dayssincelastincineration" -> Dayssincelastincineration(Dayssincelastincineration),
    "championcallout" -> Championcallout(Championcallout),
    "availablechampionbets" -> Availablechampionbets(Availablechampionbets),
    "attributes" -> Attributes(Attributes),
    "playerstatsheet" -> Playerstatsheet(Playerstatsheet),
    "tributes" -> Tributes(Tributes),
    "stream" -> Stream(StreamEntityWrapper<thisidisstaticyo::StreamData>)
}
