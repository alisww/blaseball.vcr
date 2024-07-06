use blaseball_vcr::vhs::tributes::recorder::JsonTributes;
use blaseball_vcr::{RawChroniclerEntity, VCRResult};
use clap::{clap_app, ArgMatches};
use indicatif::{MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use new_encoder::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use uuid::Uuid;

#[tokio::main]
pub async fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 tributes downloader")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for tape")
    )
    .get_matches();

    run(matches).await
}

async fn run(matches: ArgMatches<'_>) -> VCRResult<()> {
    let client = reqwest::Client::new();

    let bars = MultiProgress::new();
    bars.set_alignment(MultiProgressAlignment::Top);
    let entity_ids = vec![Uuid::nil().to_string()];

    println!("| found {} entities", entity_ids.len());

    let bar_style = ProgressStyle::default_bar()
        .template("{msg:.bold} - {pos}/{len} {wide_bar:40.green/white}")
        .unwrap();

    let entity_id_bar = bars.add(ProgressBar::new(entity_ids.len() as u64));
    entity_id_bar.set_style(bar_style.clone());
    entity_id_bar.set_message("encoding entities");

    entity_id_bar.tick();
    entity_id_bar.set_message(format!("encoding {}", Uuid::nil().to_string()));

    let versions: Vec<RawChroniclerEntity<JsonTributes>> = v2_paged_get(
        &client,
        "https://api.sibr.dev/chronicler/v2/versions",
        &bars,
        ChroniclerParameters {
            next_page: None,
            entity_type: "tributes".to_owned(),
            id: None,
            order: Some("asc".to_owned()),
            count: 1000,
            at: None,
            before: None,
            game_id: None
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

    let mut out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?);
    for version in versions {
        out.write_all(&serde_json::to_vec(&version).unwrap())?;
        out.write_all(b"\n")?;
    }

    out.flush()?;
    // let mut dict =
    // let mut header = Vec::new();
    // recorder.finish(&mut header)?;

    // println!("header length: {}", header.len());
    // println!("store length: {}", tributes_out.into_inner().len());
    // let (mut header, mut main) = recorder.finish()?;
    // let out = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    // use std::io::Seek;
    // header.rewind()?;
    // main.rewind()?;

    // merge_tape(header, main, dict.as_deref(), out)?;

    Ok(())
}
