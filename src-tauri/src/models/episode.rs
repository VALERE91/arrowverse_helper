use crate::schema::episodes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::macros::format_description;

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i32,
    pub name: String,
    pub overview: Option<String>,
    pub episode_number: i32,
    pub air_date: String,
    pub still_url: Option<String>,
    pub plex_url: Option<String>,
}

impl Episode {
    pub fn to_dbepisode(&self) -> anyhow::Result<DBEpisode> {
        let format = format_description!("[year]-[month]-[day]");
        Ok(DBEpisode {
            id: None,
            episode_number: self.episode_number,
            episode_id: self.id,
            season_id: 0,
            name: self.name.clone(),
            air_date: time::Date::parse(&self.air_date, format)?,
            overview: self.overview.clone(),
            still_path: self.still_url.clone(),
            plex_path: self.plex_url.clone(),
            watched: false,
            watched_date: None,
        })
    }
}

#[derive(Queryable, Identifiable, Insertable, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(super::season::DBSeason))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = episodes)]
pub struct DBEpisode {
    pub id: Option<i32>,
    pub episode_id: i32,
    pub episode_number: i32,
    pub season_id: i32,
    pub name: String,
    pub air_date: time::Date,
    pub overview: Option<String>,
    pub still_path: Option<String>,
    pub plex_path: Option<String>,
    pub watched: bool,
    pub watched_date: Option<time::Date>,
}

impl DBEpisode {
    pub fn to_episode(&self) -> Episode {
        Episode {
            id: self.episode_id,
            name: self.name.clone(),
            overview: self.overview.clone(),
            episode_number: self.episode_number,
            air_date: self.air_date.to_string(),
            still_url: self.still_path.clone(),
            plex_url: self.plex_path.clone(),
        }
    }
}
