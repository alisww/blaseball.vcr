use blaseball_vcr::vhs::recorder::*;
use blaseball_vcr::{timestamp_to_nanos, VCRResult};
use new_encoder::*;
use uuid::Uuid;
use vcr_schemas::game::GameUpdate;
use vcr_schemas::DynamicEntityType;

use std::fs::File;
use std::io::{self, BufReader, Read};

use clap::clap_app;
use zstd::bulk::Decompressor;

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 games encoder")
        (@arg ENTITY_LOCATION_TABLE: +required -t --table [LOCATION_DB] "database file for entity locations by entity hash")
        (@arg INPUT: +required -i --input [INPUT] "input games file")
        (@arg OUTPUT: +required -o --output [FILE] "output file for tape")
        (@arg ZSTD_DICT: +required -d --dict [DICT] "set dict for tape")
        (@arg COMPRESSION_LEVEL: -l --level [LEVEL] "set compression level")
    )
    .get_matches();

    let mut reader = BufReader::new(File::open(matches.value_of("INPUT").unwrap())?);
    let mut decompressor = Decompressor::new()?;

    let games_dict = std::fs::read(matches.value_of("ZSTD_DICT").unwrap())?;

    let mut hash_db = redb::Database::create(matches.value_of("ENTITY_LOCATION_TABLE").unwrap())?;
    let write_txn = hash_db.begin_write()?;
    let mut hash_table = write_txn.open_table(HASH_TO_ENTITY_TABLE)?;

    let mut recorder: TapeRecorder<GameUpdate, File> = TapeRecorder::new(
        DynamicEntityType::GameUpdate,
        tempfile::tempfile()?,
        Some(games_dict.clone()),
        matches
            .value_of("COMPRESSION_LEVEL")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(11),
        u16::MAX as usize,
    )?;

    let mut i = 0;

    loop {
        let mut len_buf: [u8; 8] = [0; 8];
        if let Err(e) = reader.read_exact(&mut len_buf) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                break;
            } else {
                return Err(blaseball_vcr::VCRError::IOError(e));
            }
        }

        let compressed_len = u64::from_le_bytes(len_buf);
        reader.read_exact(&mut len_buf)?;
        let decompressed_len = u64::from_le_bytes(len_buf);

        let mut buf: Vec<u8> = vec![0; compressed_len as usize];
        reader.read_exact(&mut buf)?;
        let decompressed = decompressor.decompress(&buf, decompressed_len as usize)?;

        let game_data: Vec<ChronV1GameUpdate<GameUpdate>> =
            serde_json::from_slice(&decompressed[..]).unwrap();

        let id = *uuid::Uuid::parse_str(&game_data[0].game_id)
            .unwrap()
            .as_bytes();

        let (times, data): (Vec<i64>, Vec<GameUpdate>) = game_data
            .into_iter()
            .map(|v| (timestamp_to_nanos(v.timestamp), v.data))
            .unzip();

        if times.is_empty() {
            continue;
        }

        let entity = TapeEntity { times, data, id };

        recorder.add_entity(entity, &mut hash_table)?;

        i += 1;

        println!("game #{i} - {}", Uuid::from_bytes(id));
    }

    let (header, mut main) = recorder.finish()?;
    let out = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    drop(hash_table);
    write_txn.commit()?;
    hash_db.compact()?;

    use std::io::Seek;
    main.rewind()?;

    merge_tape(header, main, Some(&games_dict[..]), out)?;

    Ok(())
}
