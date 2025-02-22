use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use error::CacheLoadError;

use crate::api::schema::Schema;
use crate::app::cache::{Cacheable, DEFAULT_CACHE_ROOT};

pub struct APILoader<S: Schema> {
    pub schema: S,
    pub cache_root: PathBuf,
}

impl<S> APILoader<S>
where
    S: Schema,
{
    pub fn new(schema: S) -> Self {
        Self {
            schema,
            cache_root: DEFAULT_CACHE_ROOT.to_path_buf(),
        }
    }

    pub fn schema(&self) -> &S {
        &self.schema
    }

    pub fn set_cache_root(&mut self, cache_root: PathBuf) -> &mut Self {
        self.cache_root = cache_root;
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

    pub fn load_cache(&self) -> Result<S::Response, CacheLoadError>
    where
        S: Cacheable,
    {
        use CacheLoadError::*;
        println!("Load cache: {:?}", self.cache_file_path());

        let path = self.cache_file_path();

        let file = File::open(&path).map_err(|e| FileNotFound(e.into()))?;
        let time_stamp = get_timestamp(&file).map_err(|e| FileNotFound(e.into()))?;

        let cache_living = (time_stamp.elapsed()).is_ok_and(|t| t < self.schema.min_interval());
        if cache_living {
            serde_json::from_reader(BufReader::new(file))
                .map_err(|e| CacheLoadError::ParseFailed(e.into()))
        } else {
            Err(CacheExpired {
                path,
                updated: time_stamp,
                interval: self.schema.min_interval(),
            })
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
        match self.load_cache() {
            Ok(response) => return Ok(response),
            Err(e) => {
                if let CacheLoadError::ParseFailed(_) = e {
                    return Err(Box::new(e));
                }
            }
        }

        let api_call = self.call_api().await?.bytes().await?;
        self.save_cache(&api_call)?;
        Ok(serde_json::from_slice(
            &self.schema.formatter().format(&api_call),
        )?)
    }
}

fn get_timestamp(file: &File) -> Result<SystemTime, std::io::Error> {
    file.metadata()?.modified()
}

pub mod error {
    pub use super::*;

    use std::fmt::{Display, Formatter, Result};
    use std::time::{Duration, SystemTime};

    #[derive(Debug)]
    pub enum CacheLoadError {
        FileNotFound(Box<dyn std::error::Error>),
        ParseFailed(Box<dyn std::error::Error>),
        CacheExpired {
            path: PathBuf,
            updated: SystemTime,
            interval: Duration,
        },
    }

    impl Display for CacheLoadError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                CacheLoadError::FileNotFound(e) => write!(f, "File not found: {}", e),
                CacheLoadError::ParseFailed(e) => write!(f, "Parse failed: {}", e),
                CacheLoadError::CacheExpired {
                    path,
                    updated,
                    interval,
                } => {
                    write!(
                        f,
                        "Cache expired: {:?} ({:?} + {:?})",
                        path, updated, interval
                    )
                }
            }
        }
    }

    impl Error for CacheLoadError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                CacheLoadError::FileNotFound(e) => Some(&**e),
                CacheLoadError::ParseFailed(e) => Some(&**e),
                CacheLoadError::CacheExpired { .. } => None,
            }
        }
    }
}
