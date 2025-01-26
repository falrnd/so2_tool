//! https://mutoys.com/so2/info/api

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub mod item_definition;
pub mod people;

fn api_cache_root() -> &'static Path {
    Path::new(r"data\api\cache")
}

fn create_cache_dir() {
    let _ = std::fs::create_dir_all(api_cache_root());
}

pub fn get_file_path<P: AsRef<Path>>(item_filename: P) -> PathBuf {
    create_cache_dir();
    Path::new(api_cache_root()).join(item_filename)
}

pub const fn api_call_default_interval() -> Duration {
    Duration::from_secs(3600)
}

struct APICall<Response> {
    // API endpoint & cache file name
    pub endpoint: String,
    pub cache_file_path: PathBuf,

    pub interval: Duration,

    _phantom: std::marker::PhantomData<Response>,
}

impl<Response> APICall<Response>
where
    Response: for<'de> serde::de::Deserialize<'de> + serde::ser::Serialize,
{
    pub fn new(endpoint: &str, cache_file_name: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            cache_file_path: get_file_path(cache_file_name),
            interval: api_call_default_interval(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set_interval(self, interval: Duration) -> Self {
        Self { interval, ..self }
    }

    async fn api_call(&self) -> Result<String, reqwest::Error> {
        println!("API call: {}", self.endpoint);
        reqwest::get(&self.endpoint).await?.text().await
    }

    pub async fn load_cache_or_call(self) -> Result<Response, Box<dyn std::error::Error>> {
        if self.cache_file_path.exists() {
            let file = File::open(&self.cache_file_path)?;
            let file_last_modified = file.metadata()?.modified()?;
            if file_last_modified.elapsed()? < self.interval {
                let cache = serde_json::from_reader(BufReader::new(file))?;
                return Ok(cache);
            }
        }

        let api_call = self.api_call().await?;
        File::create(&self.cache_file_path)?.write_all(api_call.as_bytes())?;
        Ok(serde_json::from_str(&api_call)?)
    }
}
