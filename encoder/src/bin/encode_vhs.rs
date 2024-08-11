use blaseball_vcr::etypes::DynamicEntityType;
use blaseball_vcr::vhs::recorder::*;
use blaseball_vcr::{RawChroniclerEntity, VCRResult};
use borsh::BorshSerialize;
use clap::{clap_app, ArgMatches};
use indicatif::{MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use new_encoder::*;
use std::fs::File;
use std::str::FromStr;
use vcr_schemas::*;

#[tokio::main]
pub async fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 encoder")
        (@arg ENTITY_LOCATION_TABLE: -t --table [LOCATION_DB] "database file for entity locations by entity hash")
        (@arg CHECKPOINT_EVERY: -c --checkpoints [CHECKPOINT_FREQUENCY] "how often should the diff engine create a checkpoint it can skip to?")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for tape")
        (@arg ZSTD_DICT: -d --dict [DICT] "set dict for tape")
        (@arg ENTITY: +required <TYPE> ... "entity type to encode")
    )
    .get_matches();

    let etype: String = matches.value_of("ENTITY").unwrap().to_owned();

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
        "playerstatsheet" > Playerstatsheet,
        "sponsordata" > Sponsordata
    )
}

async fn run<
    T: vhs_diff::Diff
        + vhs_diff::Patch
        + Send
        + Sync
        + serde::de::DeserializeOwned
        + serde::Serialize
        + Clone
        + BorshSerialize,
>(
    etype: String,
    matches: ArgMatches<'_>,
) -> VCRResult<()> {
    let client = reqwest::Client::new();

    let dict = if let Some(path) = matches.value_of("ZSTD_DICT") {
        Some(std::fs::read(path)?)
    } else {
        None
    };

    let mut recorder: TapeRecorder<T, File> = TapeRecorder::new(
        DynamicEntityType::from_str(&etype).unwrap(),
        tempfile::tempfile()?,
        dict.clone(),
        matches
            .value_of("COMPRESSION_LEVEL")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(11),
        matches
            .value_of("CHECKPOINT_EVERY")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(u16::MAX as usize),
    )?;

    let mut hash_db = redb::Database::create(matches.value_of("ENTITY_LOCATION_TABLE").unwrap())?;
    let write_txn = hash_db.begin_write()?;
    let mut hash_table = write_txn.open_table(HASH_TO_ENTITY_TABLE)?;

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
            game_id: None,
            at: None,
            before: None,
        },
    )
    .await?
    .into_iter()
    .map(|e| e.entity_id)
    .collect();
    // let entity_ids = vec![String::from("4304bcf9-239a-4aa2-a410-56a487217a2a")];

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

        let entity_versions: Vec<ChroniclerEntity<T>> = v2_paged_get(
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
                before: None,
                game_id: None,
            },
        )
        .await?
        .into_iter()
        .map(|v| RawChroniclerEntity {
            entity_id: v.entity_id,
            hash: v.hash,
            valid_from: v.valid_from,
            valid_to: v.valid_to,
            data: serde_json::from_value(v.data).unwrap(),
        })
        .collect();

        if !entity_versions.is_empty() {
            recorder.add_entity(TapeEntity::from(entity_versions), &mut hash_table)?;
        }
    }

    let (header, mut main) = recorder.finish()?;
    let out = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    drop(hash_table);
    write_txn.commit()?;
    hash_db.compact()?;

    use std::io::Seek;
    main.rewind()?;

    merge_tape(header, main, dict.as_deref(), out)?;

    Ok(())
}
