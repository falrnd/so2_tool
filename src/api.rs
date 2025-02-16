//! https://mutoys.com/so2/info/api

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::sync::LazyLock;

use cache::Cacheable;
use schema::Schema;

pub mod cache;
pub mod schema;

static DEFAULT_CACHE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| r"data\api\cache".into());

pub struct APICall<Request: Schema + Cacheable> {
    pub cache_root: PathBuf,
    pub request: Request,
}

impl<Request> APICall<Request>
where
    Request: Schema + Cacheable,
    <Request as Schema>::Response: for<'de> serde::de::Deserialize<'de>,
{
    pub fn new(request: Request) -> Self {
        Self {
            cache_root: DEFAULT_CACHE_ROOT.clone(),
            request,
        }
    }

    pub fn set_cache_root(&mut self, cache_root: PathBuf) -> &mut Self {
        self.cache_root = cache_root;
        self
    }

    fn cache_file_path(&self) -> PathBuf {
        self.cache_root.join(self.request.file_path())
    }

    async fn api_call(self) -> Result<String, reqwest::Error> {
        let endpoint = self.request.endpoint();
        println!("API call: {}", endpoint);
        reqwest::get(endpoint).await?.text().await
    }

    pub async fn load_cache_or_call(self) -> Result<Request::Response, Box<dyn std::error::Error>> {
        let cache_file_path = self.cache_file_path();

        if cache_file_path.exists() {
            let file = File::open(&cache_file_path)?;
            let file_last_modified = file.metadata()?.modified()?;
            if file_last_modified.elapsed()? < self.request.min_interval() {
                let cache = serde_json::from_reader(BufReader::new(file))?;
                return Ok(cache);
            }
        }

        let api_call = self.api_call().await?;
        let _ = std::fs::create_dir_all(cache_file_path.parent().unwrap());
        File::create(&cache_file_path)?.write_all(api_call.as_bytes())?;
        Ok(serde_json::from_str(&api_call)?)
    }
}
