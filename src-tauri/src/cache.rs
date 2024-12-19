use diesel::prelude::*;

use crate::{
    db,
    models::{
        episode::{self, DBEpisode},
        season::{self, DBSeason},
        show::{self, DBShow, TVShow},
    },
    tmdb,
};

pub struct Cache {
    conn: SqliteConnection,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            conn: db::establish_connection(),
        }
    }

    pub async fn prepare_cache(&mut self) -> anyhow::Result<()> {
        let arrowverse_shows = tmdb::get_series();
        for serie in arrowverse_shows {
            let show = serie.get_infos().await?;
            self.cache_show(&show)?;
        }
        Ok(())
    }

    pub fn get_all_shows(&mut self) -> anyhow::Result<Vec<TVShow>> {
        use crate::schema::series::dsl::*;

        Ok(series
            .select(DBShow::as_select())
            .load(&mut self.conn)?
            .iter()
            .map(|show| show.to_tvshow())
            .collect())
    }

    fn cache_show(&mut self, show: &TVShow) -> anyhow::Result<()> {
        use crate::schema::series::dsl::*;

        let mut results = series
            .filter(series_id.eq(show.id as i32))
            .limit(1)
            .select(DBShow::as_select())
            .load(&mut self.conn)?;

        if results.is_empty() {
            let db_show = show.to_dbshow();
            results = vec![diesel::insert_into(series)
                .values(&db_show)
                .returning(DBShow::as_returning())
                .get_result(&mut self.conn)?];
        }

        let db_show = &results[0];
        for season in &show.seasons {
            self.cache_season(&season, db_show)?;
        }

        Ok(())
    }

    fn cache_season(&mut self, season: &season::Season, show: &DBShow) -> anyhow::Result<()> {
        use crate::schema::seasons::dsl::*;

        if show.id.is_none() {
            return Err(anyhow::anyhow!("Show ID is missing"));
        }

        let mut results = seasons
            .filter(season_id.eq(season.id as i32))
            .limit(1)
            .select(DBSeason::as_select())
            .load(&mut self.conn)?;

        if results.is_empty() {
            let mut db_season = season.to_dbseason();
            db_season.series_id = show.id.unwrap();
            results = vec![diesel::insert_into(seasons)
                .values(&db_season)
                .returning(DBSeason::as_returning())
                .get_result(&mut self.conn)?];
        }

        let db_season = &results[0];
        for episode in &season.episodes {
            self.cache_episodes(episode, db_season)?;
        }

        Ok(())
    }

    fn cache_episodes(
        &mut self,
        episode: &episode::Episode,
        season: &DBSeason,
    ) -> anyhow::Result<()> {
        use crate::schema::episodes::dsl::*;

        if season.id.is_none() {
            return Err(anyhow::anyhow!("Season ID is missing"));
        }

        let results = episodes
            .filter(episode_id.eq(episode.id as i32))
            .limit(1)
            .select(DBEpisode::as_select())
            .load(&mut self.conn)?;

        if results.is_empty() {
            let mut db_episode = episode.to_dbepisode()?;
            db_episode.season_id = season.id.unwrap();
            vec![diesel::insert_into(episodes)
                .values(&db_episode)
                .returning(DBEpisode::as_returning())
                .get_result(&mut self.conn)?];
        }

        Ok(())
    }
}
