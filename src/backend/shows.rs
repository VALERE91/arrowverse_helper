pub struct TVShow {
    pub name: String,
    pub poster: String,
}

impl TVShow {
    pub fn get_all() -> anyhow::Result<Vec<Self>> {
        Ok(vec![
            TVShow {
                name: "Arrow".to_string(),
                poster: "https://image.tmdb.org/t/p/w500/6moGODTcbw8vH8kY8V5bKurWt0Z.jpg"
                    .to_string(),
            },
            TVShow {
                name: "The Flash".to_string(),
                poster: "https://image.tmdb.org/t/p/w500/wHa6KOJAoNTFLFtp7wguUJKSnju.jpg"
                    .to_string(),
            },
            TVShow {
                name: "Supergirl".to_string(),
                poster: "https://image.tmdb.org/t/p/w500/vqBsgL9nd2v04ZvCqPzwtckDdFD.jpg"
                    .to_string(),
            },
        ])
    }
}
