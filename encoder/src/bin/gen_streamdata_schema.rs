#![feature(buf_read_has_data_left, path_add_extension)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    sync::mpsc::sync_channel,
};

use blaseball_vcr::VCRResult;
use clap::clap_app;
use genson_rs::SchemaBuilder;
use simd_json::prelude::*;

fn main() -> VCRResult<()> {
    let matches = clap_app!(train_vhs_dict =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr streamdata splitter")
        (@arg INPUT: +required -i --input [FILE] "streamdata directory")
        (@arg OUTPUT: +required -o --output [FILE] "output directory")
    )
    .get_matches();

    let out_path = PathBuf::from_str(matches.value_of("OUTPUT").unwrap()).unwrap();

    for file in std::fs::read_dir(matches.value_of("INPUT").unwrap())? {
        let file = file?.path();
        if !file.is_file() {
            continue;
        }

        println!("Generating schema for {}", file.display());

        let n_workers = 4;

        let (object_tx, object_rx) = crossbeam::channel::bounded(4096 * 8);
        let (schema_tx, schema_rx) = crossbeam::channel::bounded(n_workers);

        crossbeam::scope(|s| {
            // producer
            s.spawn(|_| -> VCRResult<()> {
                let mut reader = BufReader::new(zstd::Decoder::new(File::open(&file)?)?);
                let mut buffers = simd_json::Buffers::new(1_000_000);
                let mut line_buffer = String::new();
                let mut i = 0;

                while reader.has_data_left()? {
                    if i % 100 == 0 {
                        println!("READ #{i} (channel len {})", object_tx.len());
                    }

                    reader.read_line(&mut line_buffer)?;
                    line_buffer.pop();
                    if let Ok(mut v) = simd_json::to_borrowed_value_with_buffers(
                        unsafe { line_buffer.as_bytes_mut() },
                        &mut buffers,
                    ) {
                        let obj = v.as_object_mut().unwrap();
                        let data = obj.remove("data").unwrap();
                        object_tx.send(data.into_static()).unwrap();
                    }

                    line_buffer.clear();

                    i += 1;
                }

                drop(object_tx);

                Ok(())
            });

            for _ in 0..n_workers {
                let (object_rx, schema_tx) = (object_rx.clone(), schema_tx.clone());

                s.spawn(move |_| {
                    let mut schema_builder = SchemaBuilder::new(None);

                    for object in object_rx {
                        schema_builder.add_object(&object);
                    }

                    schema_tx.send(schema_builder.to_schema()).unwrap();
                });
            }

            drop(schema_tx);

            let mut merged_schema = SchemaBuilder::new(None);

            for (i, object) in schema_rx.iter().enumerate() {
                let i = i + 1;
                println!("merging schema {i}/{n_workers}");

                merged_schema.add_schema(object);
            }

            std::fs::write(
                out_path
                    .join(file.file_name().unwrap())
                    .with_added_extension("schema"),
                merged_schema.to_json().as_bytes(),
            )
            .unwrap();
        })
        .unwrap();
    }

    Ok(())
}
