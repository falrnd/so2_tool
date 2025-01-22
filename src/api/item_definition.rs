use std::fs::File;
use std::io::{Read, Write};

use super::{api_call_default_interval, get_file_path};

pub const ENDPOINT: &str = "https://so2-api.mutoys.com/master/item.json";

const ITEMS_FILE_NAME: &str = r"item_definition.json";

async fn api_call() -> Result<String, reqwest::Error> {
    eprintln!("API call");
    reqwest::get(ENDPOINT).await?.text().await
}

/// todo: serde
pub struct ItemDefinition {
    pub value: String,
}

pub async fn get() -> Result<ItemDefinition, Box<dyn std::error::Error>> {
    let file_path = get_file_path(ITEMS_FILE_NAME);

    if file_path.exists() {
        let mut file = File::open(&file_path)?;
        let file_last_modified = file.metadata()?.modified()?;
        if file_last_modified.elapsed()? < api_call_default_interval() {
            if let Ok(cache) = deserialize(&mut file) {
                return Ok(ItemDefinition { value: cache });
            }
        }
    }

    let api_call = api_call();
    let mut file = File::create(&file_path)?;
    let api_call = api_call.await?;
    file.write_all(api_call.as_bytes())?;
    Ok(ItemDefinition { value: api_call })
}

fn deserialize(file: &mut File) -> Result<String, Box<dyn std::error::Error>> {
    // todo: serde
    let mut cache = String::new();
    file.read_to_string(&mut cache)?;
    Ok(cache)
}
