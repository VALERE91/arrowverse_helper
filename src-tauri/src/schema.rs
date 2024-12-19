// @generated automatically by Diesel CLI.

diesel::table! {
    episodes (id) {
        id -> Nullable<Integer>,
        episode_id -> Integer,
        season_id -> Integer,
        episode_number -> Integer,
        name -> Text,
        air_date -> Date,
        overview -> Nullable<Text>,
        still_path -> Nullable<Text>,
        plex_path -> Nullable<Text>,
        watched -> Bool,
        watched_date -> Nullable<Date>,
    }
}

diesel::table! {
    seasons (id) {
        id -> Nullable<Integer>,
        season_id -> Integer,
        series_id -> Integer,
        season_number -> Integer,
        name -> Text,
        overview -> Nullable<Text>,
        poster_path -> Nullable<Text>,
    }
}

diesel::table! {
    series (id) {
        id -> Nullable<Integer>,
        series_id -> Integer,
        name -> Text,
        poster_path -> Nullable<Text>,
    }
}

diesel::joinable!(episodes -> seasons (season_id));
diesel::joinable!(seasons -> series (series_id));

diesel::allow_tables_to_appear_in_same_query!(
    episodes,
    seasons,
    series,
);
