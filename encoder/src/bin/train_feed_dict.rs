use blaseball_vcr::feed::event::FeedEvent;
use blaseball_vcr::feed::recorder::FeedDictTrainer;
use blaseball_vcr::feed::BlockMetadata;
use blaseball_vcr::VCRResult;
use clap::clap_app;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr feed dict generator")
        (@arg INPUT: +required -i --input [FILE] "feed file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for the trained dictionary")
    )
    .get_matches();

    let mut trainer = FeedDictTrainer::new();

    let mut input = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?).lines();

    let mut chunk = Vec::with_capacity(255);
    let mut current_meta = BlockMetadata {
        tournament: i8::MIN,
        season: i8::MIN,
        phase: u8::MAX,
    };

    let mut i = 0;
    let mut evi = 0;

    loop {
        let Some(line) = input.next() else { break };

        let event = serde_json::from_str::<FeedEvent>(&line?)?;
        let new_meta = BlockMetadata::from_event(&event);

        if !chunk.is_empty() && (chunk.len() >= 255 || current_meta != new_meta) {
            println!("processing chunk #{i} (event #{evi})");
            if chunk.len() >= 255 {
                println!("reason: chunklen")
            } else if current_meta.season != new_meta.season {
                println!("reason: season")
            } else if current_meta.phase != new_meta.phase {
                println!("reason: phase");
            } else if current_meta.tournament != new_meta.tournament {
                println!("reason: tournament")
            }

            trainer.add_chunk(std::mem::take(&mut chunk));
            i += 1;
        }

        current_meta = new_meta;
        chunk.push(event);

        evi += 1;
    }

    std::fs::write(matches.value_of("OUTPUT").unwrap(), trainer.train(112_000)?)?;

    Ok(())
}
