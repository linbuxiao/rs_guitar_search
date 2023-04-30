use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GuitarScore {
    pub title: String,
    pub imgs: Vec<String>,
    pub source_url: String,
}
