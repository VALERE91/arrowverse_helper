use std::env;

use super::{
    models::{season::SeasonDetails, serie::SerieDetails},
    Arrowverse,
};
use reqwest::header::{HeaderMap, AUTHORIZATION};

const BASE_URL: &str = "https://api.themoviedb.org/3/tv";
const IMG_URL: &str = "https://image.tmdb.org/t/p/w500";

pub async fn get_serie_details(serie: &Arrowverse) -> anyhow::Result<SerieDetails> {
    let id = serie.get_id();
    let url = format!("{BASE_URL}/{id}?language=en-US");
    let token = env::var("TMDB_TOKEN")?;
    let authorization = format!("Bearer {token}");

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Create custom headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization.parse()?);

    // Send the GET request with headers
    let response = client
        .get(url.clone())
        .headers(headers.clone())
        .send()
        .await?
        .json::<SerieDetails>()
        .await?;

    Ok(response)
}

pub async fn get_season_details(
    serie: &Arrowverse,
    season_number: i64,
) -> anyhow::Result<SeasonDetails> {
    let id = serie.get_id();
    let url = format!("{BASE_URL}/{id}/season/{season_number}?language=en-US");
    let token = env::var("TMDB_TOKEN")?;
    let authorization = format!("Bearer {token}");

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Create custom headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization.parse()?);

    // Send the GET request with headers
    let response = client
        .get(url.clone())
        .headers(headers.clone())
        .send()
        .await?
        .json::<SeasonDetails>()
        .await?;

    Ok(response)
}

pub fn get_img_url(path: &str) -> String {
    format!("{IMG_URL}/{path}")
}
