use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use super::APICall;

pub const ENDPOINT: &str = "https://so2-api.mutoys.com/json/people/all.json";

const ITEMS_FILE_NAME: &str = r"people.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub unit: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub area_id: u32,
    #[serde(rename = "isPositive")]
    pub is_positive: bool,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeopleOfTown {
    pub area_id: u32,
    pub unit: u32,
    pub persons: HashMap<String, Person>,
    pub trend: Option<Vec<Trend>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleResponse(Vec<PeopleOfTown>);

impl PeopleResponse {
    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {
        APICall::new(ENDPOINT, ITEMS_FILE_NAME)
            .set_interval(Duration::from_secs(600))
            .load_cache_or_call()
            .await
    }

    pub fn values(&self) -> impl Iterator<Item = &PeopleOfTown> {
        self.0.iter()
    }
}
