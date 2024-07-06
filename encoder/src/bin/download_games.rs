use blaseball_vcr::vhs::recorder::*;
use blaseball_vcr::{timestamp_to_nanos, VCRResult};
use indicatif::{MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};
use new_encoder::*;
use uuid::Uuid;
use vcr_schemas::game::GameUpdate;

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt, BufReader, BufWriter};

use clap::clap_app;
use zstd::bulk::{Compressor, Decompressor};

#[tokio::main]
async fn main() -> VCRResult<()> {
    let matches = clap_app!(download_games =>
        (version: "1.0")
        (author: "emily signet <emily@sibr.dev>")
        (about: "blaseball.vcr gen 2 games downloader")
        (@arg INPUT: +required -i --input [FILE] "game input")
        (@arg OUTPUT: +required -o --output [FILE] "output file for games")
    )
    .get_matches();

    let mut out = BufWriter::new(File::create(matches.value_of("OUTPUT").unwrap()).await?);
    let bars = MultiProgress::new();
    bars.set_alignment(MultiProgressAlignment::Top);
    let client = reqwest::Client::new();


    let bar_style = ProgressStyle::default_bar()
        .template("{msg:.bold} - {pos}/{len} {wide_bar:40.green/white}")
        .unwrap();

    let games: ChroniclerV1Response<ChronV1Game> = serde_json::from_slice(&std::fs::read(matches.value_of("INPUT").unwrap())?)?;
    let games = games.data;

    println!("found {}", games.len());

    let downloads_bar = bars.add(ProgressBar::new(games.len() as u64).with_style(bar_style).with_message("downloading games"));

    let mut compressor = Compressor::new(8)?;

    for game in games {
        downloads_bar.set_message(format!("downloading {}", game.game_id));
        let game = v1_get_game_updates(&client, "https://api.sibr.dev/chronicler/v1/games/updates", game.game_id, &bars).await?;
        
        let data = serde_json::to_vec(&game)?;
        let compressed = compressor.compress(&data)?;

        out.write_u64_le(compressed.len() as u64).await?;
        out.write_u64_le(data.len() as u64).await?;
        out.write_all(&compressed).await?;

        downloads_bar.inc(1);
    }
    
    // let mut compressor = games.
    // let mut len_buf: [u8; 8] = [0; 8];
    // if let Err(e) = reader.read_exact(&mut len_buf) {
    //     if e.kind() == io::ErrorKind::UnexpectedEof {
    //         break;
    //     } else {
    //         return Err(blaseball_vcr::VCRError::IOError(e));
    //     }
    // }

    // let compressed_len = u64::from_le_bytes(len_buf);
    // reader.read_exact(&mut len_buf)?;
    // let decompressed_len = u64::from_le_bytes(len_buf);

    // let mut buf: Vec<u8> = vec![0; compressed_len as usize];
    // reader.read_exact(&mut buf)?;
    // let decompressed = decompressor.decompress(&buf, decompressed_len as usize)?;

    // let game_data: Vec<ChronV1GameUpdate<GameUpdate>> =
    //     serde_json::from_slice(&decompressed[..]).unwrap();

    Ok(())
}