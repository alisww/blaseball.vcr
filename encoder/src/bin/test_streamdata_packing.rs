#![feature(buf_read_has_data_left)]

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use assert_json_diff::assert_json_eq;
use blaseball_vcr::{
    call_method_by_type, db_manager::DatabaseManager, db_wrapper, stream_data::{thisidisstaticyo::{self, PackedStreamData}, PackedStreamComponent, StreamComponent}, timestamp_from_millis, timestamp_to_millis, timestamp_to_nanos, vhs::recorder::HASH_TO_ENTITY_TABLE, EntityLocation, VCRResult
};
use clap::clap_app;
use humansize::{format_size, DECIMAL};
use iso8601_timestamp::Timestamp;
use redb::ReadOnlyTable;
use serde::Deserialize;
use uuid::Uuid;
use vcr_schemas::*;

#[derive(Deserialize)]
struct ChronStreamItem {
    valid_from: Timestamp,
    valid_to: Timestamp,
    data: thisidisstaticyo::StreamDataWrapper,
}

const BATCH_SIZE: usize = 100;

struct StreamDataRecorder {
    compresor: zstd::bulk::Compressor<'static>,
    manager: DatabaseManager,
    table: ReadOnlyTable<u128, EntityLocation>,
    lens: Vec<usize>,
    internal_buffer: Vec<PackedStreamData>,
    output: BufWriter<File>
}


impl StreamDataRecorder {
    fn write_item(&mut self, stream: ChronStreamItem) -> VCRResult<()> {
        let time: i64 = timestamp_to_nanos(stream.valid_from);
        let original_version = stream.data.value.unwrap();
        self.internal_buffer.push(original_version.pack(time, &self.table, &self.manager)?);
        if self.internal_buffer.len() >= BATCH_SIZE {
            self.flush()?;
        }

        Ok(())
    }

    fn flush(&mut self) -> VCRResult<()> {
        let data = self.internal_buffer.drain(..).collect::<Vec<_>>();

        let encoded_data = bitcode::encode(&data);
        let compressed_data = self.compresor.compress(&encoded_data)?;
        self.output.write_all(&(encoded_data.len() as u64).to_le_bytes())?;
        self.output.write_all(&(compressed_data.len() as u64).to_le_bytes())?;
        self.output.write_all(&compressed_data)?;
        self.lens.push(compressed_data.len());

        Ok(())
    }

    fn report(&mut self) {
        if self.lens.is_empty() {
            return;
        }

        self.lens.sort_unstable();
        let total = self.lens.iter().sum::<usize>();
        let average =  (total as f64) / self.lens.len() as f64;
        let median = self.lens[self.lens.len() / 2];

        println!("Total Size: {} | Average: {} | Median: {}", format_size(total, DECIMAL), format_size(average.round() as u64, DECIMAL), format_size(median, DECIMAL))

    }
}

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr streamdata packing tester")
        (@arg INPUT: +required -i --input [FILE] "streamdata file")
        (@arg TAPES: +required -v --vhs [TAPES]  "vhs tapes")
        (@arg ENTITY_LOCATION_TABLE: +required -t --table [TABLE] "entity location table")
        (@arg OUTPUT: +required -o --output [TABLE] "streamdata output file")

    )
    .get_matches();

    let hash_db = redb::Database::open(matches.value_of("ENTITY_LOCATION_TABLE").unwrap())?;
    let read_txn: redb::ReadTransaction = hash_db.begin_read()?;
    let hash_table = read_txn.open_table(HASH_TO_ENTITY_TABLE)?;

    let mut db_manager = DatabaseManager::new();
    for entry in std::fs::read_dir(matches.value_of("TAPES").unwrap()).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let stem = path.file_stem().unwrap().to_string_lossy().to_owned();
            println!("-> loading {}", stem);
            call_method_by_type!(
                db_wrapper::from_single_and_insert,
                (&mut db_manager, &entry.path()),
                stem.as_ref(),
                { continue }
            )
            .unwrap();
        }
    }

    let file = File::open(matches.value_of("INPUT").unwrap())?;

    let mut reader = BufReader::new(zstd::Decoder::new(&file)?);

    let mut i = 0;
    let mut line_buffer = String::new();

    let mut writer = StreamDataRecorder {
        compresor: zstd::bulk::Compressor::new(8)?,
        manager: db_manager,
        table: hash_table,
        lens: Vec::with_capacity(2_000_000),
        internal_buffer: Vec::with_capacity(BATCH_SIZE),
        output: BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap())?),
    };

    let mut json_buffers = simd_json::Buffers::new(5_000_000);

    while reader.has_data_left()? {
        if i % 100 == 0 {
            println!("READ #{i}");
        }

        if i % 100_000 == 0 {
            println!("READ #{i}");
            writer.report();
        }

        reader.read_line(&mut line_buffer)?;

        line_buffer.pop();

        if line_buffer.is_empty() {
            continue;
        }

            let value: ChronStreamItem = unsafe { simd_json::serde::from_str_with_buffers(&mut line_buffer, &mut json_buffers).unwrap() };
            writer.write_item(value).unwrap();

        line_buffer.clear();

        i += 1;
    }

    writer.flush()?;

    writer.report();

    Ok(())
}
