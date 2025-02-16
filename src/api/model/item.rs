use std::collections::HashMap;

use itertools::Itertools;
use serde::Deserialize;

pub struct Request {}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(flatten)]
    value: HashMap<String, Item>,
}

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

impl Response {
    pub fn values(&self) -> impl Iterator<Item = &Item> {
        self.value.values().sorted_by_key(|item| item.sort)
    }

    pub fn into_values(self) -> impl Iterator<Item = Item> {
        self.value.into_values().sorted_by_key(|item| item.sort)
    }
}
