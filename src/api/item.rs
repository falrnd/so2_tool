use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::APICall;

pub const ENDPOINT: &str = "https://so2-api.mutoys.com/master/item.json";

const ITEMS_FILE_NAME: &str = r"item.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub category: String,
    pub class: String,
    pub item_id: u32,
    pub limit: u32,
    pub name: String,
    pub scale: String,
    pub sort: u32,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{}) ", self.name, self.category, self.class)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemResponse {
    #[serde(flatten)]
    value: HashMap<String, Item>,
}

impl ItemResponse {
    pub fn values(&self) -> impl Iterator<Item = &Item> {
        self.value.values().sorted_by_key(|item| item.sort)
    }

    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {
        APICall::new(ENDPOINT, ITEMS_FILE_NAME)
            .load_cache_or_call()
            .await
    }
}
