mod api;
mod models;

use api::{get_img_url, get_season_details};

use crate::models::{episode::Episode, season::Season, show::TVShow};

#[derive(Debug)]
pub enum Arrowverse {
    Arrow,
    Flash,
    Supergirl,
    DCLegends,
    Constantine,
    Vixen,
    BlackLightning,
    Batwoman,
    Stargirl,
    SupermanAndLois,
}

impl Arrowverse {
    pub fn get_id(&self) -> i32 {
        match self {
            Arrowverse::Arrow => 1412,
            Arrowverse::Flash => 60735,
            Arrowverse::Supergirl => 62688,
            Arrowverse::DCLegends => 62643,
            Arrowverse::Constantine => 60743,
            Arrowverse::Vixen => 62125,
            Arrowverse::BlackLightning => 71663,
            Arrowverse::Batwoman => 89247,
            Arrowverse::Stargirl => 80986,
            Arrowverse::SupermanAndLois => 95057,
        }
    }

    pub async fn get_infos(&self) -> anyhow::Result<TVShow> {
        let detail = api::get_serie_details(self).await?;

        // Extract all the seasons
        let mut seasons: Vec<Season> = vec![];
        for season in &detail.seasons {
            if season.season_number == 0 {
                continue;
            }
            let mut season_info = Season {
                id: season.id,
                season_number: season.season_number,
                overview: season.overview.clone(),
                episodes: vec![],
                poster_url: Some(get_img_url(&season.poster_path)),
            };

            let season_detail = get_season_details(self, season.season_number).await?;

            for episode in season_detail.episodes {
                season_info.episodes.push(Episode {
                    name: episode.name,
                    id: episode.id as i32,
                    overview: Some(episode.overview),
                    episode_number: episode.episode_number as i32,
                    air_date: episode.air_date,
                    still_url: Some(get_img_url(&episode.still_path)),
                    plex_url: None,
                });
            }

            seasons.push(season_info);
        }

        let show = TVShow {
            id: detail.id,
            name: detail.name,
            seasons,
            poster_url: Some(get_img_url(&detail.poster_path)),
        };
        Ok(show)
    }
}

pub fn get_series() -> Vec<Arrowverse> {
    vec![
        Arrowverse::Arrow,
        Arrowverse::Flash,
        Arrowverse::Supergirl,
        Arrowverse::DCLegends,
        Arrowverse::Constantine,
        Arrowverse::Vixen,
        Arrowverse::BlackLightning,
        Arrowverse::Batwoman,
        Arrowverse::Stargirl,
        Arrowverse::SupermanAndLois,
    ]
}
