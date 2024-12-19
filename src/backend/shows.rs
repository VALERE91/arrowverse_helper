use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::invoke_without_args;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TVShow {
    pub id: i32,
    pub name: String,
    pub poster: String,
}

impl TVShow {
    pub async fn get_all() -> Vec<Self> {
        let shows = invoke_without_args("shows").await;
        let shows_array: Vec<HashMap<String, String>> =
            serde_wasm_bindgen::from_value(shows).unwrap();

        for show in shows_array {
            println!("{:?}", show);
        }
        vec![]
    }
}
