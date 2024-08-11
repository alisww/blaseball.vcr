#![feature(get_mut_unchecked)]
// mod paging;
// use blaseball_vcr::db_manager::*;
// use blaseball_vcr::*;
// use paging::*;

// fn main() -> VCRResult<()> {

//     let mut page = Page::entities(
//         u32::MAX,
//         db_manager.all_entity_ids::<Player>().unwrap().to_vec(),
//     );

//     let mut total = 0;

//     loop {
//         let v = page.take_n::<Player>(&db_manager, 100).unwrap();
//         if v.is_empty() {
//             break;
//         }

//         println!("iter:  got {}", v.len());
//         total += v.len();
//     }

//     println!("end: got {total}");

//     Ok(())
// }

#[macro_use]
extern crate rocket;

use blaseball_vcr::feed::db::FeedDatabase;
use blaseball_vcr::site::AssetManager;
use blaseball_vcr::stream_data::db::StreamDatabase;
use blaseball_vcr::stream_data::thisidisstaticyo;
use blaseball_vcr::vhs::tributes::TributesDatabase;
use serde::Serialize;

mod fairings;
mod feed;
mod paging;
mod site;
mod v1;
mod v2;

use fairings::*;
use paging::*;
use v2::*;

use blaseball_vcr::db_manager::DatabaseManager;
use blaseball_vcr::{call_method_by_type, db_wrapper, EntityDatabase};
use rocket::figment::{
    providers::{Env, Format, Toml},
    value::magic::RelativePathBuf,
    Figment, Profile,
};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use vcr_schemas::*;

#[derive(Serialize)]
pub struct ChronV1Response<T> {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    data: Vec<T>,
}

#[derive(Serialize)]
pub struct ChronResponse<T> {
    #[serde(rename = "nextPage")]
    next_page: Option<String>,
    #[serde(rename = "items")]
    data: Vec<T>,
}

pub type DynChronResponse = ChronResponse<DynamicChronEntity>;

#[cfg(not(feature = "bundle_before"))]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    rocket::custom(figment)
}

#[cfg(feature = "bundle_before")]
async fn build_rocket(figment: Figment) -> rocket::Rocket<rocket::Build> {
    use rocket::figment::{providers::Serialized, util::map};

    let profile = Profile::from_env_or("VCR_PROFILE", "default");
    let figment = figment
        .merge(Serialized::from(
            &map![
                "chronicler_base_url" => "{addr}/vcr/",
                "upnuts_base_url" => "{addr}/vcr/",
            ],
            profile.as_str(),
        ))
        .merge(Serialized::from(
            &map![
                "siesta_mode" => true,
                "chronplete" => true,
            ],
            profile.as_str(),
        ));

    before::build(&figment)
        .await
        .map_err(|e| e.downcast::<rocket::figment::Error>())
        .unwrap()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if let Some((_, path)) = std::env::vars().find(|(k, _)| k == "APPDIR") {
        std::env::set_current_dir(path).unwrap();
    } else {
        // traverse from the directory where we live up until we find a Vcr.toml, then chdir there.
        if let Ok(dir) = std::env::current_exe() {
            if let Some(new_dir) = dir.ancestors().find(|d| d.join("Vcr.toml").exists()) {
                std::env::set_current_dir(new_dir).unwrap();
            }
        }
    };

    #[derive(serde::Deserialize, Debug)]
    struct VCRConfig {
        stream_tape: Option<RelativePathBuf>,
        tapes_folder: Option<RelativePathBuf>,
        feed_tape: Option<RelativePathBuf>,
        feed_indexes: Option<RelativePathBuf>,
        site_tape: Option<RelativePathBuf>,
        tributes_tape: Option<RelativePathBuf>,
        #[serde(default)]
        check_site_tapes: bool,
        #[serde(default)]
        time_requests: bool,
        #[serde(default)]
        cache: CacheConfig,
    }

    #[derive(serde::Deserialize, Debug)]
    struct CacheConfig {
        size: u64,
        #[serde(with = "humantime_serde")]
        time_to_idle: Duration,
    }

    impl Default for CacheConfig {
        fn default() -> CacheConfig {
            CacheConfig {
                size: 256,
                time_to_idle: Duration::from_secs(10 * 60),
            }
        }
    }

    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Vcr.toml").nested())
        .merge(Env::prefixed("VCR_"))
        .select(Profile::from_env_or("VCR_PROFILE", "default"));

    let config: VCRConfig = figment.extract_inner("vcr").expect("missing vcr config!");

    let mut rocket = build_rocket(figment).await;
    let mut db_manager = DatabaseManager::new();

    for entry in std::fs::read_dir(
        config
            .tapes_folder
            .map(|v| v.relative())
            .unwrap_or(PathBuf::from("./vhs_tapes")),
    )
    .unwrap()
    {
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

    if let Some(tributes) = config.tributes_tape {
        let db = TributesDatabase::from_single(tributes.relative()).unwrap();
        db_manager.insert(db);
    }

    let mut db_manager = Arc::new(db_manager);

    if let Some(stream_file) = config.stream_tape {
        let file = File::open(stream_file.relative()).unwrap();
        let stream: StreamDatabase<
            thisidisstaticyo::StreamData,
            thisidisstaticyo::PackedStreamData,
        > = StreamDatabase::initialize(file, Arc::clone(&db_manager)).unwrap();
        // girls love unsafely mutating a shared value
        unsafe {
            Arc::<blaseball_vcr::db_manager::DatabaseManager>::get_mut_unchecked(&mut db_manager)
                .insert(stream)
        };
    }

    if let Some(site_path) = config.site_tape {
        let site_manager = AssetManager::from_single(site_path.relative()).unwrap();
        if config.check_site_tapes {
            for asset_kind in site_manager.assets.keys() {
                let (total, failures) = site_manager.check_asset(asset_kind).unwrap();
                println!(
                    "{} ~ {} checksum matches ({} failures)",
                    asset_kind,
                    total - failures.len(),
                    failures.len()
                );
            }
        }

        rocket = rocket
            .manage(site_manager)
            .mount("/vcr", routes![site::site_updates, site::site_download]);
    }

    if let Some(feed_path) = config.feed_tape {
        let feed_db = FeedDatabase::from_tape(
            feed_path.relative(),
            config.feed_indexes.map(|v| v.relative()),
        )
        .unwrap();
        rocket = rocket
            .manage(feed_db)
            .mount("/vcr", routes![feed::api::feed]);
    }

    if config.time_requests {
        rocket = rocket.attach(RequestTimer);
    }

    let rocket = rocket
        .manage(db_manager)
        .manage(PageManager::new(
            config.cache.size,
            config.cache.time_to_idle,
        ))
        .mount(
            "/vcr",
            routes![entities, versions, v1::games, v1::game_updates,],
        );
    rocket.launch().await?;
    Ok(())
}
