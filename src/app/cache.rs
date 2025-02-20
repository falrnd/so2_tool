use std::{path::Path, sync::LazyLock};

use crate::api::schema::{AreaSummary, OfficialItem, People, RecipeItem, ShopSummary};

pub static DEFAULT_CACHE_ROOT: LazyLock<&Path> = LazyLock::new(|| Path::new(r"data\api\cache"));

pub trait Cacheable {
    fn file_path(&self) -> impl AsRef<Path>;
}

impl Cacheable for OfficialItem {
    fn file_path(&self) -> impl AsRef<Path> {
        "item.json"
    }
}

impl Cacheable for RecipeItem {
    fn file_path(&self) -> impl AsRef<Path> {
        "recipe_item.json"
    }
}

impl Cacheable for ShopSummary {
    fn file_path(&self) -> impl AsRef<Path> {
        "shop_summary.json"
    }
}

impl Cacheable for People {
    fn file_path(&self) -> impl AsRef<Path> {
        "people.json"
    }
}

impl Cacheable for AreaSummary {
    fn file_path(&self) -> impl AsRef<Path> {
        "area_summary.json"
    }
}
