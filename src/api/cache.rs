use std::{path::Path, sync::LazyLock};

use crate::api::schema::{item, people};

pub static DEFAULT_CACHE_ROOT: LazyLock<&Path> = LazyLock::new(|| Path::new(r"data\api\cache"));

pub trait Cacheable {
    fn file_path(&self) -> impl AsRef<Path>;
}

impl Cacheable for item::Request {
    fn file_path(&self) -> impl AsRef<Path> {
        "item.json"
    }
}

impl Cacheable for people::Request {
    fn file_path(&self) -> impl AsRef<Path> {
        "people.json"
    }
}
