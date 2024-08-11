#![feature(buf_read_has_data_left)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};

use blaseball_vcr::VCRResult;
use clap::clap_app;
use iso8601_timestamp::Timestamp;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use simd_json::base::ValueAsScalar;
use uuid::Uuid;
use zstd::{Decoder, Encoder};

#[derive(Deserialize)]
struct SeasonBoundary {
    sim: String,
    season: String,
    start: Timestamp,
    end: Timestamp,
}

// {
//     "hash": "5379114b-3890-a3da-9692-5dc2867e174e",
//     "version_id": "7eda9a23-9263-4f22-ad46-e1af040cbf64",
//     "entity_id": "00000000-0000-0000-0000-000000000000",
//     "valid_from": "2021-06-22T09:24:26.347388+00:00",
//     "valid_to": "2021-06-22T09:24:31.357984+00:00",
//     "seq": 1663228,
//     "type": 3,
//     "data":

#[derive(Serialize, Deserialize)]
struct StreamItem<'a> {
    hash: Uuid,
    version_id: Uuid,
    entity_id: Uuid,
    valid_from: Timestamp,
    valid_to: Timestamp,
    seq: i64,
    #[serde(rename = "type")]
    etype: i64,
    #[serde(borrow)]
    data: &'a RawValue,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr streamdata splitter")
        (@arg INPUT: +required -i --input [FILE] "streamdata file")
        (@arg OUTPUT: +required -o --output [FILE] "output directory")
    )
    .get_matches();

    let mut seasons: Vec<SeasonBoundary> =
        serde_json::from_str(include_str!("../../seasons.json"))?;
    seasons.sort_by_key(|s| s.start);

    let out_folder = PathBuf::from_str(matches.value_of("OUTPUT").unwrap()).unwrap();

    let mut season_files = {
        let mut sim_values: Vec<String> = seasons.iter().map(|v| v.sim.clone()).collect();
        sim_values.sort();
        sim_values.dedup();

        let mut season_files = HashMap::with_capacity(seasons.len());

        for sim in sim_values {
            let f = BufWriter::new(File::create(
                out_folder.join(format!("{}.ndjson.zst", sim)),
            )?);
            let writer = Encoder::new(f, 6)?;

            season_files.insert(sim, writer);
        }

        season_files
    };

    let valid_from_regex = Regex::new(r#"\"valid_from\":\".+?\""#).unwrap();

    let mut decoder = BufReader::new(Decoder::new(File::open(
        matches.value_of("INPUT").unwrap(),
    )?)?);
    let mut i = 0;

    let mut json_buffers = simd_json::Buffers::new(1_000_000);
    let mut line_buffer = String::with_capacity(1_000_000);

    while decoder.has_data_left()? {
        if i % 100 == 0 {
            println!("#{i}");
        }

        decoder.read_line(&mut line_buffer)?;
        let preserved_line = line_buffer.clone();

        let data: simd_json::BorrowedValue = simd_json::to_borrowed_value_with_buffers(
            unsafe { line_buffer.as_bytes_mut() },
            &mut json_buffers,
        )
        .unwrap();
        let timestamp = data["valid_from"].as_str().unwrap();

        let timestamp: Timestamp = Timestamp::parse(timestamp).unwrap();

        let season_by_time = match seasons.binary_search_by_key(&timestamp, |s| s.start) {
            Ok(i) => i,
            Err(i) => i,
        };

        let file = season_files.get_mut(&seasons[season_by_time].sim).unwrap();

        file.write_all(preserved_line.as_bytes())?;
        file.write_all(b"\n")?;

        drop(data);

        i += 1;

        line_buffer.clear();
    }
    // for line in decoder.lines() {
    //     println!("#{i}");
    //     let mut line = line?;

    // }

    for file in season_files.into_values() {
        let mut file = file.finish()?;
        file.flush()?;
        let file = file.into_inner().unwrap();
        file.sync_all()?;
    }

    Ok(())
}
