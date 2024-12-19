use diesel::prelude::*;

use crate::{
    db,
    models::{
        episode::{self, DBEpisode},
        season::{self, DBSeason},
        show::{DBShow, TVShow},
    },
    tmdb,
};

pub async fn prepare_cache() -> anyhow::Result<()> {
    let mut conn = db::establish_connection();

    let arrowverse_shows = tmdb::get_series();
    for serie in arrowverse_shows {
        let show = serie.get_infos().await?;
        cache_show(&mut conn, &show)?;
    }
    Ok(())
}

fn cache_show(conn: &mut SqliteConnection, show: &TVShow) -> anyhow::Result<()> {
    use crate::schema::series::dsl::*;

    let mut results = series
        .filter(series_id.eq(show.id as i32))
        .limit(1)
        .select(DBShow::as_select())
        .load(conn)?;

    if results.is_empty() {
        let db_show = show.to_dbshow();
        results = vec![diesel::insert_into(series)
            .values(&db_show)
            .returning(DBShow::as_returning())
            .get_result(conn)?];
    }

    let db_show = &results[0];
    for season in &show.seasons {
        cache_season(conn, &season, db_show)?;
    }

    Ok(())
}

fn cache_season(
    conn: &mut SqliteConnection,
    season: &season::Season,
    show: &DBShow,
) -> anyhow::Result<()> {
    use crate::schema::seasons::dsl::*;

    if show.id.is_none() {
        return Err(anyhow::anyhow!("Show ID is missing"));
    }

    let mut results = seasons
        .filter(season_id.eq(season.id as i32))
        .limit(1)
        .select(DBSeason::as_select())
        .load(conn)?;

    if results.is_empty() {
        let mut db_season = season.to_dbseason();
        db_season.series_id = show.id.unwrap();
        results = vec![diesel::insert_into(seasons)
            .values(&db_season)
            .returning(DBSeason::as_returning())
            .get_result(conn)?];
    }

    let db_season = &results[0];
    for episode in &season.episodes {
        cache_episodes(conn, episode, db_season)?;
    }

    Ok(())
}

fn cache_episodes(
    conn: &mut SqliteConnection,
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
        .load(conn)?;

    if results.is_empty() {
        let mut db_episode = episode.to_dbepisode()?;
        db_episode.season_id = season.id.unwrap();
        vec![diesel::insert_into(episodes)
            .values(&db_episode)
            .returning(DBEpisode::as_returning())
            .get_result(conn)?];
    }

    Ok(())
}
