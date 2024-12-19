use crate::schema::seasons;

use super::episode::Episode;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: i64,
    pub season_number: i64,
    pub overview: String,
    pub episodes: Vec<Episode>,
    pub poster_url: Option<String>,
}

impl Season {
    pub fn to_dbseason(&self) -> DBSeason {
        DBSeason {
            id: None,
            series_id: 0,
            season_id: self.id as i32,
            season_number: self.season_number as i32,
            name: format!("Season {}", self.season_number),
            overview: Some(self.overview.clone()),
            poster_path: self.poster_url.clone(),
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(super::show::DBShow))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = seasons)]
pub struct DBSeason {
    pub id: Option<i32>,
    pub season_id: i32,
    pub series_id: i32,
    pub season_number: i32,
    pub name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}

impl DBSeason {
    pub fn to_season(&self) -> Season {
        Season {
            id: self.season_id as i64,
            season_number: self.name.split(' ').last().unwrap().parse().unwrap(),
            overview: self.overview.clone().unwrap_or_default(),
            episodes: vec![],
            poster_url: self.poster_path.clone(),
        }
    }
}
