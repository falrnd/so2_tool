use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::api::schema::Schema;
use crate::app::cache::{Cacheable, DEFAULT_CACHE_ROOT};

pub struct APILoader<S: Schema> {
    pub schema: S,
    pub cache_root: PathBuf,
    pub allow_cache_expired: bool,
}

impl<S> APILoader<S>
where
    S: Schema,
{
    pub fn new(schema: S) -> Self {
        Self {
            schema,
            cache_root: DEFAULT_CACHE_ROOT.to_path_buf(),
            allow_cache_expired: false,
        }
    }

    pub fn schema(&self) -> &S {
        &self.schema
    }

    pub fn set_cache_root(&mut self, cache_root: PathBuf) -> &mut Self {
        self.cache_root = cache_root;
        self
    }

    pub fn allow_cache_expired(&mut self, y: bool) -> &mut Self {
        self.allow_cache_expired = y;
        self
    }

    fn cache_file_path(&self) -> PathBuf
    where
        S: Cacheable,
    {
        self.cache_root.join(self.schema.file_path())
    }

    pub async fn call_api(&self) -> Result<reqwest::Response, reqwest::Error> {
        let endpoint = self.schema.endpoint();
        println!("API call: {}", endpoint);
        reqwest::get(endpoint).await
    }

    pub fn load_cache(&self) -> Result<S::Response, Box<dyn Error>>
    where
        S: Cacheable,
    {
        println!("Load cache: {:?}", self.cache_file_path());

        let cache_file_path = self.cache_file_path();
        let file = File::open(&cache_file_path)?;
        let time_stamp;

        if self.allow_cache_expired || {
            time_stamp = file.metadata()?.modified()?;
            time_stamp.elapsed()? < self.schema.min_interval()
        } {
            Ok(serde_json::from_reader(BufReader::new(file))?)
        } else {
            Err(Box::new(error::CacheExpired::new(
                cache_file_path,
                time_stamp,
                self.schema.min_interval(),
            )))
        }
    }

    pub fn save_cache(&self, api_call: &[u8]) -> Result<(), Box<dyn Error>>
    where
        S: Cacheable,
    {
        let cache_file_path = self.cache_file_path();
        let _ = std::fs::create_dir_all(cache_file_path.parent().expect("invalid cache dir"));
        File::create(&cache_file_path)?.write_all(api_call)?;
        println!("Save cache: {:?}", cache_file_path);
        Ok(())
    }

    pub async fn get(&self) -> Result<S::Response, Box<dyn Error>>
    where
        S: Cacheable,
    {
        if let Ok(v) = self.load_cache() {
            return Ok(v);
        }

        let api_call = self.call_api().await?.bytes().await?;
        self.save_cache(&api_call)?;
        Ok(serde_json::from_slice(
            &self.schema.formatter().format(&api_call),
        )?)
    }
}

pub mod error {
    pub use super::*;

    use std::fmt::{Display, Formatter, Result};
    use std::time::{Duration, SystemTime};

    #[derive(Debug)]
    pub struct CacheExpired {
        pub file: PathBuf,
        pub updated: SystemTime,
        pub interval: Duration,
    }

    impl CacheExpired {
        pub fn new(file: PathBuf, updated: SystemTime, interval: Duration) -> Self {
            Self {
                file,
                updated,
                interval,
            }
        }

        pub fn expired_at(&self) -> SystemTime {
            self.updated + self.interval
        }
    }

    impl Display for CacheExpired {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(
                f,
                "Cache file {:?} expired at {:?}",
                self.file,
                self.expired_at()
            )
        }
    }

    impl Error for CacheExpired {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}
