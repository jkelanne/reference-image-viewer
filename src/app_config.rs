//use serde::{Serialize, Deserialize};
use serde_derive::{Deserialize, Serialize};
//use serde_json::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub last_open_location: String,
}

impl AppConfig {
    pub fn empty() -> Self {
        Self {
            last_open_location: String::new(),
        }
    }
}
