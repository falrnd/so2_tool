use std::collections::HashMap;

use itertools::Itertools;
use serde::Deserialize;

use crate::api::APICall;

pub const ENDPOINT: &str = "https://so2-api.mutoys.com/master/item.json";

const ITEMS_FILE_NAME: &str = r"item.json";

#[derive(Debug, Clone, Deserialize)]
pub struct Category(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Class(pub String);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackSize(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Scale(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub category: Category,
    pub class: Class,
    pub item_id: Id,
    pub limit: StackSize,
    pub name: Name,
    pub scale: Scale,
    pub sort: u32,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{}) ", self.name.0, self.category.0, self.class.0)
    }
}

impl std::hash::Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
    }
}

#[derive(Debug, Deserialize)]
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
