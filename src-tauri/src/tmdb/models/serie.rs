use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerieDetails {
    pub adult: bool,
    #[serde(rename = "backdrop_path")]
    pub backdrop_path: String,
    #[serde(rename = "created_by")]
    pub created_by: Vec<CreatedBy>,
    #[serde(rename = "episode_run_time")]
    pub episode_run_time: Vec<i64>,
    #[serde(rename = "first_air_date")]
    pub first_air_date: String,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: i64,
    #[serde(rename = "in_production")]
    pub in_production: bool,
    pub languages: Vec<String>,
    #[serde(rename = "last_air_date")]
    pub last_air_date: String,
    #[serde(rename = "last_episode_to_air")]
    pub last_episode_to_air: LastEpisodeToAir,
    pub name: String,
    #[serde(rename = "next_episode_to_air")]
    pub next_episode_to_air: Value,
    pub networks: Vec<Network>,
    #[serde(rename = "number_of_episodes")]
    pub number_of_episodes: i64,
    #[serde(rename = "number_of_seasons")]
    pub number_of_seasons: i64,
    #[serde(rename = "origin_country")]
    pub origin_country: Vec<String>,
    #[serde(rename = "original_language")]
    pub original_language: String,
    #[serde(rename = "original_name")]
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    #[serde(rename = "poster_path")]
    pub poster_path: String,
    #[serde(rename = "production_companies")]
    pub production_companies: Vec<ProductionCompany>,
    #[serde(rename = "production_countries")]
    pub production_countries: Vec<ProductionCountry>,
    pub seasons: Vec<Season>,
    #[serde(rename = "spoken_languages")]
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
    #[serde(rename = "vote_count")]
    pub vote_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub id: i64,
    #[serde(rename = "credit_id")]
    pub credit_id: String,
    pub name: String,
    #[serde(rename = "original_name")]
    pub original_name: String,
    pub gender: i64,
    #[serde(rename = "profile_path")]
    pub profile_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEpisodeToAir {
    pub id: i64,
    pub name: String,
    pub overview: String,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
    #[serde(rename = "vote_count")]
    pub vote_count: i64,
    #[serde(rename = "air_date")]
    pub air_date: String,
    #[serde(rename = "episode_number")]
    pub episode_number: i64,
    #[serde(rename = "episode_type")]
    pub episode_type: String,
    #[serde(rename = "production_code")]
    pub production_code: String,
    pub runtime: i64,
    #[serde(rename = "season_number")]
    pub season_number: i64,
    #[serde(rename = "show_id")]
    pub show_id: i64,
    #[serde(rename = "still_path")]
    pub still_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: i64,
    #[serde(rename = "logo_path")]
    pub logo_path: Option<String>,
    pub name: String,
    #[serde(rename = "origin_country")]
    pub origin_country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionCompany {
    pub id: i64,
    #[serde(rename = "logo_path")]
    pub logo_path: Option<String>,
    pub name: String,
    #[serde(rename = "origin_country")]
    pub origin_country: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionCountry {
    #[serde(rename = "iso_3166_1")]
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    #[serde(rename = "air_date")]
    pub air_date: String,
    #[serde(rename = "episode_count")]
    pub episode_count: i64,
    pub id: i64,
    pub name: String,
    pub overview: String,
    #[serde(rename = "poster_path")]
    pub poster_path: String,
    #[serde(rename = "season_number")]
    pub season_number: i64,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpokenLanguage {
    #[serde(rename = "english_name")]
    pub english_name: String,
    #[serde(rename = "iso_639_1")]
    pub iso_639_1: String,
    pub name: String,
}
