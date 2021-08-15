use blaseball_vcr::site::{chron::SiteUpdate, manager::ResourceManager};
use blaseball_vcr::{feed::*, *};
use chrono::{DateTime, TimeZone, Utc};
use lru::LruCache;
use rand::Rng;
use rocket::figment::{
    providers::{Format, Toml},
    Figment, Profile,
};
use rocket::{
    get,
    http::{uri::Origin, ContentType, Header, Status},
    options,
    response::Redirect,
    routes,
    serde::json::Json as RocketJson,
    FromForm, State,
};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;
use uuid::Uuid;

#[cfg(feature = "bundle_before")]
use rocket::{response::content::Html, Either};

pub struct RequestTimer;

#[derive(Copy, Clone)]
struct TimerStart(Option<Instant>);

struct CORS;
#[rocket::async_trait]
impl rocket::fairing::Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "CORS headers",
            kind: rocket::fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[rocket::async_trait]
impl<'r> rocket::response::Responder<'r, 'static> for CORS {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .header(Header::new("Access-Control-Allow-Methods", "GET"))
            .header(Header::new("Access-Control-Allow-Headers", "*"))
            .header(Header::new("Access-Control-Max-Age", "86400"))
            .header(Header::new("Allow", "OPTIONS, GET"))
            .status(Status::NoContent)
            .ok()
    }
}

#[options("/<_..>")]
async fn cors_preflight() -> CORS {
    CORS
}

#[rocket::async_trait]
impl rocket::fairing::Fairing for RequestTimer {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Timer",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        request.local_cache(|| TimerStart(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, _: &mut rocket::Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(duration) = start_time.0.map(|st| st.elapsed()) {
            if let Some(route) = req.route() {
                let query_params = if let Some(query) = req.uri().query() {
                    query
                        .segments()
                        .fold(String::new(), |acc, (k, v)| format!("{}={} {}", k, v, acc))
                } else {
                    "no params".to_owned()
                };
                println!(
                    "\x1b[31;1m{}\x1b[m\x1b[1m + {}\x1b[m-> \x1b[4m{:?}\x1b[m",
                    route.name.as_ref().unwrap(),
                    query_params,
                    duration
                );
            }
        }
    }
}

#[derive(FromForm)]
struct EntityReq {
    #[field(name = "type")]
    entity_type: String,
    #[field(name = "id")]
    ids: Option<String>,
    at: Option<String>,
    count: Option<usize>,
    page: Option<String>,
    order: Option<String>,
}

#[derive(FromForm)]
struct VersionsReq {
    #[field(name = "type")]
    entity_type: String,
    #[field(name = "id")]
    ids: Option<String>,
    before: Option<String>,
    after: Option<String>,
    count: Option<usize>,
    order: Option<String>,
    page: Option<String>,
}

#[get("/v1/site/updates")]
fn site_updates(manager: &State<ResourceManager>) -> RocketJson<ChroniclerV1Response<SiteUpdate>> {
    RocketJson(ChroniclerV1Response {
        next_page: None,
        data: manager.expand_site_updates("/assets"),
    })
}

#[get("/v1/assets/<r_type>/<r_idx>")]
fn get_asset(
    r_type: &str,
    r_idx: u16,
    manager: &State<ResourceManager>,
) -> VCRResult<(ContentType, Vec<u8>)> {
    Ok((
        match r_type {
            "index" => ContentType::HTML,
            "maincss" => ContentType::CSS,
            "mainjs" | "2js" => ContentType::JavaScript,
            _ => panic!(), // TODO: result instead
        },
        manager.get_resource(r_type, r_idx)?,
    ))
}

#[get("/v1/games?<after>")]
fn all_games(
    after: Option<String>,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<ChroniclerV1Response<ChronV1Game>>> {
    Ok(RocketJson(ChroniclerV1Response {
        next_page: None,
        data: db.games_with_date(after.map_or(Utc.timestamp(0, 0), |d| {
            DateTime::parse_from_rfc3339(&d)
                .unwrap()
                .with_timezone(&Utc)
        }))?,
    }))
}

#[get("/feed/<kind>?<id>&<time>&<start>&<category>&<limit>&<phase>&<season>")]
fn feed(
    kind: &str,
    id: Option<String>,
    time: Option<i64>,
    start: Option<String>,
    limit: Option<usize>,
    phase: Option<u8>,
    season: Option<i8>,
    category: Option<i8>,
    db: &State<Mutex<FeedDatabase>>,
) -> VCRResult<RocketJson<Vec<FeedEvent>>> {
    let mut feed = db.lock().unwrap();

    let time = start
        .map(|s| s.parse::<DateTime<Utc>>().unwrap())
        .unwrap_or(time.map_or(Utc::now(), |d| Utc.timestamp_millis(d)));

    let category: i8 = category.unwrap_or(-3);

    match kind {
        "global" => {
            if phase.is_some() && season.is_some() {
                Ok(RocketJson(feed.events_by_phase(
                    season.unwrap(),
                    phase.unwrap(),
                    limit.unwrap_or(1000),
                )?))
            } else {
                Ok(RocketJson(feed.events_before(
                    time,
                    limit.unwrap_or(100),
                    category,
                )?))
            }
        }
        "player" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Player,
                limit.unwrap_or(100),
                category,
            )?))
        }
        "team" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Team,
                limit.unwrap_or(100),
                category,
            )?))
        }
        "game" => {
            Ok(RocketJson(feed.events_by_tag_and_time(
                time,
                &Uuid::parse_str(&id.ok_or(VCRError::EntityNotFound)?).unwrap(), // wrong sort of error. oop. also do n't unwrap
                TagType::Game,
                limit.unwrap_or(100),
                category,
            )?))
        }
        _ => Err(VCRError::EntityTypeNotFound),
    }
}

#[get("/feed/story?<time>&<id>")]
fn library(
    time: Option<i64>,
    id: &str,
    db: &State<MultiDatabase>,
) -> VCRResult<RocketJson<Vec<JSONValue>>> {
    Ok(RocketJson(
        serde_json::from_value::<Vec<JSONValue>>(
            db.get_entity(
                "librarystory",
                id,
                time.map_or(Utc::now().timestamp() as u32, |d| {
                    Utc.timestamp_millis(d).timestamp() as u32
                }),
            )?
            .data,
        )
        .unwrap(),
    ))
}

#[derive(Debug)]
struct StreamDataStep(u32);

#[get("/v2/versions?<req..>")]
fn versions(
    req: VersionsReq,
    step: &State<StreamDataStep>,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res: ChroniclerResponse<ChroniclerEntity> = if req.entity_type.to_lowercase()
        == "stream"
    {
        let start_time = req.after.as_ref().map_or(
            req.before.as_ref().map_or(u32::MAX, |x| {
                DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
            }) - ((req.count.unwrap_or(1) as u32) * step.0),
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );

        let end_time = req.before.map_or(
            req.after.map_or(u32::MIN, |x| {
                DateTime::parse_from_rfc3339(&x).unwrap().timestamp() as u32
            }) + ((req.count.unwrap_or(1) as u32) * step.0),
            |y| DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32,
        );

        let mut results: Vec<ChroniclerEntity> = Vec::new();
        for at in (start_time..end_time).into_iter().step_by(step.0 as usize) {
            results.push(ChroniclerEntity {
                entity_id: "00000000-0000-0000-0000-000000000000".to_owned(),
                valid_from: Utc.timestamp(at as i64, 0),
                valid_to: Some(Utc.timestamp((at + step.0) as i64, 0).to_rfc3339()),
                hash: String::new(),
                data: db.stream_data(at)?,
            });
        }

        ChroniclerResponse {
            next_page: None,
            items: results,
        }
    } else {
        if let Some(page_token) = req.page {
            let mut page_cache = page_map.lock().unwrap();
            if let Some(ref mut p) = page_cache.get_mut(&page_token) {
                let results: Vec<ChroniclerEntity> =
                    db.fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?;
                if results.len() < req.count.unwrap_or(100) {
                    ChroniclerResponse {
                        next_page: None,
                        items: results,
                    }
                } else {
                    ChroniclerResponse {
                        next_page: Some(page_token),
                        items: results,
                    }
                }
            } else {
                return Err(VCRError::InvalidPageToken);
            }
        } else {
            let start_time = req.after.as_ref().map_or(u32::MIN, |y| {
                DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
            });

            let end_time = req.before.map_or(u32::MAX, |y| {
                DateTime::parse_from_rfc3339(&y).unwrap().timestamp() as u32
            });

            let mut page = if let Some(ids) = req
                .ids
                .map(|i| i.split(",").map(|x| x.to_owned()).collect::<Vec<String>>())
            {
                InternalPaging {
                    remaining_data: vec![],
                    remaining_ids: ids,
                    kind: ChronV2EndpointKind::Versions(end_time, start_time),
                }
            } else {
                InternalPaging {
                    remaining_data: vec![],
                    remaining_ids: db.all_ids(&req.entity_type.to_lowercase())?,
                    kind: ChronV2EndpointKind::Versions(end_time, start_time),
                }
            };

            let res = db.fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?;
            if !(res.len() < req.count.unwrap_or(100)) {
                let mut page_cache = page_map.lock().unwrap();
                let key = {
                    let mut k = String::new();
                    let mut rng = rand::thread_rng();

                    loop {
                        let chars: String = std::iter::repeat(())
                            .map(|()| rng.sample(rand::distributions::Alphanumeric))
                            .map(char::from)
                            .take(16)
                            .collect();
                        if !page_cache.contains(&chars) {
                            k = chars;
                            break;
                        }
                    }

                    k
                };

                page_cache.put(key.clone(), page);

                ChroniclerResponse {
                    next_page: Some(key),
                    items: res,
                }
            } else {
                ChroniclerResponse {
                    next_page: None,
                    items: res,
                }
            }
        }
    };

    if let Some(ord) = req.order {
        if ord.to_lowercase() == "asc" {
            res.items.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            res.items.sort_by_key(|x| x.valid_from);
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}

#[get("/v2/entities?<req..>")]
fn entities(
    req: EntityReq,
    db: &State<MultiDatabase>,
    page_map: &State<Mutex<LruCache<String, InternalPaging>>>,
) -> VCRResult<RocketJson<ChroniclerResponse<ChroniclerEntity>>> {
    let mut res = if let Some(page_token) = req.page {
        let mut page_cache = page_map.lock().unwrap();
        if let Some(ref mut p) = page_cache.get_mut(&page_token) {
            let results: Vec<ChroniclerEntity> = db
                .fetch_page(&req.entity_type.to_lowercase(), p, req.count.unwrap_or(100))?
                .into_iter()
                .filter(|x| x.data != json!({}))
                .collect();
            if results.len() < req.count.unwrap_or(100) {
                ChroniclerResponse {
                    next_page: None,
                    items: results,
                }
            } else {
                ChroniclerResponse {
                    next_page: Some(page_token),
                    items: results,
                }
            }
        } else {
            return Err(VCRError::InvalidPageToken);
        }
    } else {
        let at = req.at.map_or(u32::MAX, |when| {
            DateTime::parse_from_rfc3339(&when).unwrap().timestamp() as u32
        });

        let mut page = if let Some(ids) = req
            .ids
            .map(|i| i.split(",").map(|x| x.to_owned()).collect::<Vec<String>>())
        {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: ids,
                kind: ChronV2EndpointKind::Entities(at),
            }
        } else {
            InternalPaging {
                remaining_data: vec![],
                remaining_ids: db.all_ids(&req.entity_type.to_lowercase())?,
                kind: ChronV2EndpointKind::Entities(at),
            }
        };

        let res: Vec<ChroniclerEntity> = db
            .fetch_page(
                &req.entity_type.to_lowercase(),
                &mut page,
                req.count.unwrap_or(100),
            )?
            .into_iter()
            .filter(|x| x.data != json!({}))
            .collect();
        if !(res.len() < req.count.unwrap_or(100)) {
            let mut page_cache = page_map.lock().unwrap();
            let key = {
                let mut k = String::new();
                let mut rng = rand::thread_rng();

                loop {
                    let chars: String = std::iter::repeat(())
                        .map(|()| rng.sample(rand::distributions::Alphanumeric))
                        .map(char::from)
                        .take(16)
                        .collect();
                    if !page_cache.contains(&chars) {
                        k = chars;
                        break;
                    }
                }

                k
            };

            page_cache.put(key.clone(), page);

            ChroniclerResponse {
                next_page: Some(key),
                items: res,
            }
        } else {
            ChroniclerResponse {
                next_page: None,
                items: res,
            }
        }
    };

    if let Some(ord) = req.order {
        if ord.to_lowercase() == "asc" {
            res.items.sort_by_key(|x| x.valid_from);
        } else if ord.to_lowercase() == "desc" {
            res.items.sort_by_key(|x| x.valid_from);
            res.items.reverse();
        }
    }

    Ok(RocketJson(res))
}

#[get("/database/coffee")]
fn coffee() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::Plain, "Coffee?"))
}

#[cfg(not(feature = "bundle_before"))]
#[get("/youtube/<id>")]
fn embed(id: &str, origin: &Origin) -> Redirect {
    Redirect::to(format!(
        "https://www.youtube.com/embed/{}?{}",
        id,
        origin.query().map(|q| q.as_str()).unwrap_or_default(),
    ))
}

#[cfg(feature = "bundle_before")]
#[get("/youtube/<id>")]
fn embed(
    id: &str,
    origin: &Origin,
    config: &State<before::Config>,
) -> Either<Html<String>, Redirect> {
    println!("{:?}", config.static_dir.join(format!("{}.webm", id)));
    if config.static_dir.join(format!("{}.webm", id)).exists() {
        Either::Left(Html(format!(include_str!("video.html"), id = id)))
    } else {
        Either::Right(Redirect::to(format!(
            "https://www.youtube.com/embed/{}?{}",
            id,
            origin.query().map(|q| q.as_str()).unwrap_or_default(),
        )))
    }
}

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
    before::build(&figment).await.unwrap()
}

async fn spinny(formatting: &str, msg: &str) {
    for frame in vec![
        "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]",
        "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
    ]
    .into_iter()
    .cycle()
    {
        print!("\x1b[1000D{}{} {}\x1b[0m", formatting, frame, msg);
        std::io::stdout().flush();
        rocket::tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    }
}

#[rocket::launch]
async fn build_vcr() -> rocket::Rocket<rocket::Build> {
    #[derive(serde::Deserialize)]
    struct VCRConfig {
        tapes: String,
        site_assets: String,
        zstd_dictionaries: Option<String>,
        feed: Option<FeedConfig>,
        cached_page_capacity: Option<usize>,
        entities_cache_size: Option<usize>,
        time_responses: Option<bool>,
        cors: Option<bool>,
        stream_data_step: Option<u32>,
        #[cfg(feature = "bundle_before")]
        open_in_browser: Option<bool>,
    }

    #[derive(serde::Deserialize)]
    struct FeedConfig {
        index: String,
        path: String,
        dict: String,
        id_table: String,
        tag_table: String,
        cache_size: Option<usize>,
    }

    println!("Please wait.....");

    // traverse from the directory where we live up until we find a Vcr.toml, then chdir there.
    if let Ok(dir) = std::env::current_exe() {
        if let Some(new_dir) = dir.ancestors().find(|d| d.join("Vcr.toml").exists()) {
            std::env::set_current_dir(new_dir).unwrap();
        }
    }

    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Vcr.toml").nested())
        .select(Profile::from_env_or("VCR_PROFILE", "default"));
    let config: VCRConfig = figment.extract_inner("vcr").expect("missing vcr config!");
    let mut rocket = build_rocket(figment).await;

    let dicts = if let Some(dicts_folder) = config.zstd_dictionaries {
        std::fs::read_dir(dicts_folder)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, std::io::Error>>()
            .unwrap()
            .into_iter()
            .filter_map(|path| {
                if let Some(ext) = path.extension() {
                    if ext == "dict" {
                        Some((
                            path.file_stem().unwrap().to_string_lossy().to_string(),
                            path,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<String, PathBuf>>()
    } else {
        HashMap::new()
    };

    let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading entities database"));
    let dbs = MultiDatabase::from_folder(
        PathBuf::from(config.tapes),
        dicts,
        config.entities_cache_size.unwrap_or(30),
    )
    .unwrap();
    blahaj.abort();

    println!("");

    let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading site assets"));
    let manager = ResourceManager::from_folder(&config.site_assets).unwrap();
    blahaj.abort();
    println!("");

    if let Some(feed_config) = config.feed {
        let blahaj = rocket::tokio::task::spawn(spinny("\x1b[1m", "reading feed data"));
        let feed_db = Mutex::new(
            FeedDatabase::from_files(
                feed_config.index,
                feed_config.path,
                feed_config.dict,
                feed_config.id_table,
                feed_config.tag_table,
                feed_config.cache_size.unwrap_or(50),
            )
            .unwrap(),
        );
        blahaj.abort();
        println!("");
        rocket = rocket.manage(feed_db).mount("/vcr", routes![feed]);
    }

    if config.time_responses.unwrap_or(false) {
        rocket = rocket.attach(RequestTimer);
    }

    if config.cors.unwrap_or(false) {
        rocket = rocket.attach(CORS);
    }

    let cache: LruCache<String, InternalPaging> =
        LruCache::new(config.cached_page_capacity.unwrap_or(20));

    #[cfg(feature = "bundle_before")]
    if config.open_in_browser.unwrap_or(false) {
        rocket = rocket.attach(rocket::fairing::AdHoc::on_liftoff("Open in browser", |r| {
            Box::pin(async move {
                let url = format!(
                    "{}://{}:{}",
                    if r.config().tls_enabled() {
                        "https"
                    } else {
                        "http"
                    },
                    r.config().address,
                    r.config().port,
                );
                if open::that(&url).is_err() {
                    println!("Couldn't open before in default browser");
                }
            })
        }));
    }

    rocket
        .manage(dbs)
        .manage(manager)
        .manage(Mutex::new(cache))
        .manage(StreamDataStep(config.stream_data_step.unwrap_or(5)))
        .mount(
            "/vcr",
            routes![
                all_games,
                entities,
                get_asset,
                site_updates,
                versions,
                library,
                coffee,
                embed,
                cors_preflight
            ],
        )
}
