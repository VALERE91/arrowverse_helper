use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonDetails {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "air_date")]
    pub air_date: String,
    pub episodes: Vec<Episode>,
    pub name: String,
    pub overview: String,
    #[serde(rename = "id")]
    pub id2: i64,
    #[serde(rename = "poster_path")]
    pub poster_path: String,
    #[serde(rename = "season_number")]
    pub season_number: i64,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    #[serde(rename = "air_date")]
    pub air_date: String,
    #[serde(rename = "episode_number")]
    pub episode_number: i64,
    #[serde(rename = "episode_type")]
    pub episode_type: String,
    pub id: i64,
    pub name: String,
    pub overview: String,
    #[serde(rename = "production_code")]
    pub production_code: String,
    pub runtime: i64,
    #[serde(rename = "season_number")]
    pub season_number: i64,
    #[serde(rename = "show_id")]
    pub show_id: i64,
    #[serde(rename = "still_path")]
    pub still_path: String,
    #[serde(rename = "vote_average")]
    pub vote_average: f64,
    #[serde(rename = "vote_count")]
    pub vote_count: i64,
    pub crew: Vec<Crew>,
    #[serde(rename = "guest_stars")]
    pub guest_stars: Vec<GuestStar>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub department: String,
    pub job: String,
    #[serde(rename = "credit_id")]
    pub credit_id: String,
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,
    pub name: String,
    #[serde(rename = "original_name")]
    pub original_name: String,
    pub popularity: f64,
    #[serde(rename = "profile_path")]
    pub profile_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestStar {
    pub character: String,
    #[serde(rename = "credit_id")]
    pub credit_id: String,
    pub order: i64,
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,
    pub name: String,
    #[serde(rename = "original_name")]
    pub original_name: String,
    pub popularity: f64,
    #[serde(rename = "profile_path")]
    pub profile_path: Option<String>,
}
