use std::{path::Path, sync::LazyLock};

use crate::api::schema::request::{Item, People};

pub static DEFAULT_CACHE_ROOT: LazyLock<&Path> = LazyLock::new(|| Path::new(r"data\api\cache"));

pub trait Cacheable {
    fn file_path(&self) -> impl AsRef<Path>;
}

impl Cacheable for Item {
    fn file_path(&self) -> impl AsRef<Path> {
        "item.json"
    }
}

impl Cacheable for People {
    fn file_path(&self) -> impl AsRef<Path> {
        "people.json"
    }
}
