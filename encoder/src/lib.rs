use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

pub use blaseball_vcr::RawChroniclerEntity as ChroniclerEntity;

#[macro_export]
macro_rules! etypes {
    ($e:ident, $f:ident, $m: ident, $($name:literal > $what:ty),*) => {
        match $e.as_str() {
            $(
                $name => $f::<$what>($e, $m).await,
            )*
            _ => panic!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerResponse<T> {
    pub next_page: Option<String>,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerV1Response<T> {
    pub next_page: Option<String>,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameDate {
    pub day: i32,
    pub season: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tournament: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV1Game {
    pub game_id: String,
    pub start_time: Option<iso8601_timestamp::Timestamp>,
    pub end_time: Option<iso8601_timestamp::Timestamp>,
    pub data: JSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV1GameUpdate<T> {
    pub game_id: String,
    pub timestamp: iso8601_timestamp::Timestamp,
    pub hash: String,
    pub data: T,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChroniclerParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "page")]
    pub next_page: Option<String>,
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "game")]
    pub game_id: Option<String>,
}

pub async fn v2_paged_get(
    client: &reqwest::Client,
    url: &str,
    mpb: &MultiProgress,
    mut parameters: ChroniclerParameters,
) -> anyhow::Result<Vec<ChroniclerEntity<JSONValue>>> {
    let mut results: Vec<ChroniclerEntity<JSONValue>> = Vec::new();

    let mut page = 1;
    let spinny = mpb.add(ProgressBar::new_spinner());
    spinny.enable_steady_tick(std::time::Duration::from_millis(120));
    spinny.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    loop {
        spinny.set_message(format!("downloading entities - page {}", page));
        let mut chron_response: ChroniclerResponse<ChroniclerEntity<JSONValue>> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.items);

        if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
            page += 1;
        } else {
            break;
        }
    }
    mpb.remove(&spinny);

    Ok(results)
}

pub async fn v1_get_games(
    client: &reqwest::Client,
    url: &str,
    mpb: &MultiProgress,
) -> anyhow::Result<Vec<ChronV1Game>> {
    let mut results: Vec<ChronV1Game> = Vec::with_capacity(32992);

    let mut page = 1;
    let spinny = mpb.add(ProgressBar::new_spinner());
    spinny.enable_steady_tick(std::time::Duration::from_millis(120));
    spinny.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );

    let mut parameters = ChroniclerParameters::default();
    // parameters.count = 100;

    loop {
        spinny.set_message(format!("downloading entities - page {}", page));
        let mut chron_response: ChroniclerV1Response<ChronV1Game> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.data);

        if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
            page += 1;
        } else {
            break;
        }
    }
    mpb.remove(&spinny);

    Ok(results)
}

pub async fn v1_get_game_updates(
    client: &reqwest::Client,
    url: &str,
    game_id: String,
    // mpb: &MultiProgress,
) -> anyhow::Result<Vec<ChronV1GameUpdate<JSONValue>>> {
    let mut results: Vec<ChronV1GameUpdate<JSONValue>> = Vec::with_capacity(32992);

    let mut page = 1;
    // let spinny = mpb.add(ProgressBar::new_spinner());
    // spinny.enable_steady_tick(std::time::Duration::from_millis(120));
    // spinny.set_style(
    //     ProgressStyle::default_spinner()
    //         .template("{spinner:.blue} {msg}")
    //         .unwrap(),
    // );

    let mut parameters = ChroniclerParameters::default();
    parameters.game_id = Some(game_id);
    parameters.count = 1000;

    loop {
        // spinny.set_message(format!("downloading entities - page {}", page));
        let mut chron_response: ChroniclerV1Response<ChronV1GameUpdate<JSONValue>> = client
            .get(url)
            .query(&parameters)
            .send()
            .await?
            .json()
            .await?;
        results.append(&mut chron_response.data);

        if let Some(next_page) = chron_response.next_page {
            parameters.next_page = Some(next_page);
            page += 1;
        } else {
            break;
        }
    }
    // mpb.remove(&spinny);

    Ok(results)
}
