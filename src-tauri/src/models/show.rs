use crate::schema::series;

use super::season::Season;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TVShow {
    pub id: i64,
    pub name: String,
    pub seasons: Vec<Season>,
    pub poster_url: Option<String>,
}

impl TVShow {
    pub fn to_dbshow(&self) -> DBShow {
        DBShow {
            id: None,
            series_id: self.id as i32,
            name: self.name.clone(),
            poster_path: self.poster_url.clone(),
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, Selectable, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = series)]
pub struct DBShow {
    pub id: Option<i32>,
    pub series_id: i32,
    pub name: String,
    pub poster_path: Option<String>,
}

impl DBShow {
    pub fn to_tvshow(&self) -> TVShow {
        TVShow {
            id: self.series_id as i64,
            name: self.name.clone(),
            seasons: vec![],
            poster_url: self.poster_path.clone(),
        }
    }
}
