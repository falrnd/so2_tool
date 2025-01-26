use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{api_call_default_interval, get_file_path};

pub const ENDPOINT: &str = "https://so2-api.mutoys.com/master/item.json";

const ITEMS_FILE_NAME: &str = r"item_definition.json";

async fn api_call() -> Result<ItemDefinition, reqwest::Error> {
    eprintln!("API call");
    reqwest::get(ENDPOINT).await?.json().await
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDefinition {
    #[serde(flatten)]
    value: HashMap<String, Item>,
}

impl ItemDefinition {
    pub fn values(&self) -> impl Iterator<Item = &Item> {
        self.value.values().sorted_by_key(|item| item.sort)
    }
}

pub async fn get() -> Result<ItemDefinition, Box<dyn std::error::Error>> {
    let file_path = get_file_path(ITEMS_FILE_NAME);

    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        let file_last_modified = file.metadata()?.modified()?;
        if file_last_modified.elapsed()? < api_call_default_interval() {
            match deserialize(&mut file) {
                Ok(cache) => return Ok(cache),
                Err(err) => {
                    eprintln!("Error deserializing cache: {}", err);
                }
            }
        }
    }

    let api_call = api_call().await?;
    File::create(&file_path)?.write_all(serde_json::to_string(&api_call)?.as_bytes())?;
    Ok(api_call)
}

fn deserialize(file: &mut File) -> Result<ItemDefinition, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(serde_json::from_str(&buffer)?)
}
