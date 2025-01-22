use std::path::{Path, PathBuf};
use std::time::Duration;

pub mod item_definition;

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
