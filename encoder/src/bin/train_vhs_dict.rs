use blaseball_vcr::vhs::recorder::*;
use blaseball_vcr::VCRResult;
use clap::{clap_app, ArgMatches};
use indicatif::{MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use new_encoder::*;
use serde_json::value::Value as JSONValue;

#[tokio::main]
pub async fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 dictionary trainer")
        (@arg CHECKPOINT_EVERY: -c --checkpoints [CHECKPOINT_FREQUENCY] "how often should the diff engine create a checkpoint it can skip to?")
        (@arg OUTPUT: -o --output [FILE] "set output file for dict")
        (@arg ENTITY: <TYPE> ... "entity type to encode")
    )
    .get_matches();

    let etype: String = matches.value_of("ENTITY").unwrap().to_owned();

    use vcr_schemas::*;

    new_encoder::etypes!(
        etype,
        run,
        matches,
        "gameupdate" > GameUpdate,
        "bossfight" > Bossfight,
        "communitychestprogress" > CommunityChestProgress,
        "division" > Division,
        "league" > League,
        "playoffmatchup" > Playoffmatchup,
        "playoffround" > Playoffround,
        "playoffs" > Playoffs,
        "season" > Season,
        "sim" > Sim,
        "stadium" > Stadium,
        "standings" > Standings,
        "subleague" > Subleague,
        "team" > Team,
        "sunsun" > Sunsun,
        "temporal" > Temporal,
        "tiebreakers" > Tiebreakers,
        "tournament" > Tournament,
        "bonusresult" > Bonusresult,
        "decreeresult" > Decreeresult,
        "eventresult" > Eventresult,
        "fuelprogress" > FuelProgressWrapper,
        "giftprogress" > Giftprogress,
        "globalevents" > GlobaleventsWrapper,
        "idols" > Idols,
        "item" > Item,
        "librarystory" > LibrarystoryWrapper,
        "nullified" > Nullified,
        "offseasonrecap" > Offseasonrecap,
        "offseasonsetup" > Offseasonsetup,
        "player" > Player,
        "renovationprogress" > Renovationprogress,
        "risingstars" > Risingstars,
        "shopsetup" > Shopsetup,
        "teamelectionstats" > Teamelectionstats,
        "vault" > Vault,
        "stadiumprefabs" > Stadiumprefabs,
        "thebook" > Thebook,
        "thebeat" > Thebeat,
        "teamstatsheet" > Teamstatsheet,
        "glossarywords" > Glossarywords,
        "peanutpower" > Peanutpower,
        "gammasim" > Gammasim,
        "gammaelections" > Gammaelections,
        "gammaelectionresults" > Gammaelectionresults,
        "gammaelectiondetails" > Gammaelectiondetails,
        "gammaelection" > Gammaelection,
        "gammabracket" > Gammabracket,
        "gamestatsheet" > Gamestatsheet,
        "feedseasonlist" > Feedseasonlist,
        "fanart" > Fanart,
        "dayssincelastincineration" > Dayssincelastincineration,
        "championcallout" > Championcallout,
        "availablechampionbets" > Availablechampionbets,
        "attributes" > Attributes,
        "playerstatsheet" > Playerstatsheet
    )
    // match etype.as_str() {
    //     "stadium" => run::<Stadium>(etype, matches).await,
    //     "communitychestprogress" => run::<CommunityChestProgress>(etype, matches).await,
    //     "division" => run::<Division>(etype, matches).await,
    //     "league" => run::<League>(etype, matches).await,
    //     "playoffmatchup" => run::<Playoffmatchup>(etype, matches).await,
    //     "playoffround" => run::<Playoffround>(etype, matches).await,
    //     "playoffs" => run::<Playoffs>(etype, matches).await,
    //     "season" => run::<Season>(etype, matches).await,
    //     "sim" => run::<Sim>(etype, matches).await,
    //     "standings" => run::<Standings>(etype, matches).await,
    //     "subleague" => run::<Subleague>(etype, matches).await,
    //     "sunsun" => run::<Sunsun>(etype, matches).await,
    //     "team" => run::<Team>(etype, matches).await,
    //     "temporal" => run::<Temporal>(etype, matches).await,
    //     "tiebreakers" => run::<TiebreakerWrapper>(etype, matches).await,
    //     "tournament" => run::<Tournament>(etype, matches).await,
    //     _ => panic!(),
    // }
}

async fn run<T: vhs_diff::Diff + Clone + serde::de::DeserializeOwned + serde::Serialize>(
    etype: String,
    matches: ArgMatches<'_>,
) -> VCRResult<()> {
    let client = reqwest::Client::new();
    let mut trainer = DictTrainer::new(
        matches
            .value_of("CHECKPOINT_EVERY")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(u16::MAX as usize),
    );

    let bars = MultiProgress::new();
    bars.set_alignment(MultiProgressAlignment::Top);

    let entity_ids: Vec<String> = v2_paged_get(
        &client,
        "https://api.sibr.dev/chronicler/v2/entities",
        &bars,
        ChroniclerParameters {
            next_page: None,
            entity_type: etype.clone(),
            id: None,
            order: None,
            count: 1000,
            at: None,
            before: None,
            game_id: None
        },
    )
    .await?
    .into_iter()
    .map(|e| e.entity_id)
    .collect();

    println!("| found {} entities", entity_ids.len());

    let bar_style = ProgressStyle::default_bar()
        .template("{msg:.bold} - {pos}/{len} {wide_bar:40.green/white}")
        .unwrap();

    let entity_id_bar = bars.add(ProgressBar::new(entity_ids.len() as u64));
    entity_id_bar.set_style(bar_style.clone());
    entity_id_bar.set_message("encoding entities");

    for id in entity_id_bar.wrap_iter(entity_ids.into_iter()) {
        entity_id_bar.tick();
        entity_id_bar.set_message(format!("encoding {}", id));

        let entity_versions: Vec<JSONValue> = v2_paged_get(
            &client,
            "https://api.sibr.dev/chronicler/v2/versions",
            &bars,
            ChroniclerParameters {
                next_page: None,
                entity_type: etype.clone(),
                id: Some(id.clone()),
                order: Some("asc".to_owned()),
                count: 1000,
                at: None,
                game_id: None,
                before: None
            },
        )
        .await?
        .into_iter()
        .map(|v| v.data)
        .collect();

        if !entity_versions.is_empty() {
            trainer.add_entity(
                entity_versions
                    .into_iter()
                    .map(|v| serde_json::from_value::<T>(v).unwrap())
                    .collect::<Vec<T>>(),
            )?;
        }
    }

    entity_id_bar.finish_with_message("done!");

    println!("training dict");

    let dict = trainer.train(112000)?;

    std::fs::write(matches.value_of("OUTPUT").unwrap(), dict)?;

    Ok(())
}
