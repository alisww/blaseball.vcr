#![feature(buf_read_has_data_left, path_add_extension)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    sync::mpsc::sync_channel,
};

use blaseball_vcr::stream_data::thisidisstaticyo;
use blaseball_vcr::VCRResult;
use clap::clap_app;
use genson_rs::SchemaBuilder;
use serde::Deserialize;
use simd_json::prelude::*;

#[derive(Deserialize)]
struct ChronStreamItem {
    data: thisidisstaticyo::StreamDataWrapper,
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr streamdata parsing tester")
        (@arg INPUT: +required -i --input [FILE] "streamdata file")
        (@arg KNOWN_GOOD: --knowngood [LINE] "last line known to parse")
    )
    .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;

    let mut reader = BufReader::new(zstd::Decoder::new(&file)?);
    let mut line_buffer = String::new();
    let mut i = 0;

    // last known line that parsers
    let known_good: usize = matches
        .value_of("KNOWN_GOOD")
        .map(|v| v.parse::<usize>().unwrap())
        .unwrap_or_default();

    while reader.has_data_left()? {
        if i % 100 == 0 {
            println!("READ #{i}");
        }

        reader.read_line(&mut line_buffer)?;

        line_buffer.pop();

        if line_buffer.is_empty() {
            continue;
        }

        if i < known_good {
            i += 1;
            line_buffer.clear();
            continue;
        }

        let jd = &mut serde_json::Deserializer::from_str(&line_buffer);
        let result: Result<ChronStreamItem, _> = serde_path_to_error::deserialize(jd);
        if let Err(e) = result {
            println!("FAILED ON LINE #{i} - {e}");
            break;
        }

        line_buffer.clear();

        i += 1;
    }

    Ok(())
}
