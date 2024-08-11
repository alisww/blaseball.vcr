use blaseball_vcr::feed::event::FeedEvent;
use blaseball_vcr::feed::recorder::FeedRecorder;
use blaseball_vcr::feed::BlockMetadata;
use blaseball_vcr::vhs::recorder::merge_tape;
use blaseball_vcr::VCRResult;
use clap::clap_app;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 feed encoder")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
        (@arg ZSTD_DICT: -d --dict [DICT] "set dict for tape")
        (@arg INPUT: +required -i --input [FILE] "feed file (ndjson)")
        (@arg OUTPUT: +required -o --output [FILE] "set output file for tape")
        (@arg AUX_OUTPUT: +required -a --aux [FILE] "set output file for uuid -> internal id lookup table")
    )
    .get_matches();

    let dict = if let Some(path) = matches.value_of("ZSTD_DICT") {
        Some(std::fs::read(path)?)
    } else {
        None
    };

    let mut recorder: FeedRecorder<File, File> = FeedRecorder::new(
        tempfile::tempfile()?,
        File::create(matches.value_of("AUX_OUTPUT").unwrap())?,
        dict.clone(),
        matches
            .value_of("COMPRESSION_LEVEL")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(11),
    )?;

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

            recorder.add_chunk(std::mem::take(&mut chunk), current_meta)?;
            i += 1;
        }

        current_meta = new_meta;
        chunk.push(event);

        evi += 1;
    }

    // for (i, line_chunk) in input.lines().chunks(255).into_iter().enumerate() {
    //     println!("processing chunk #{i}");

    //     let mut chunk = Vec::with_capacity(255);

    //     for line in line_chunk {
    //         chunk.push(serde_json::from_str::<FeedEvent>(&line?)?);
    //     }

    //     recorder.add_chunk(chunk)?;
    // }

    let (header, mut main, mut aux) = recorder.finish()?;
    let out = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    aux.flush()?;

    use std::io::Seek;
    // header.rewind()?;
    main.rewind()?;

    merge_tape(header, BufReader::new(main), dict.as_deref(), out)?;

    Ok(())
}

// fn flush_chunk(recorder: &mut FeedRecorder<impl Write, impl Write>, chunk: &mut Vec<FeedEvent>, meta: BlockMetadata) -> VCRResult<()> {

//     Ok(())
// }
