use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::api::schema::{
    AreaSummary, OfficialItem, People, RecipeItem, RequestReport, Schema, ShopSummary,
};

pub static DEFAULT_CACHE_ROOT: LazyLock<&Path> = LazyLock::new(|| Path::new(r"data\api\cache"));

pub trait Cacheable: Schema {
    // use for delete expired cache
    fn file_dir() -> Option<impl AsRef<Path>> {
        Option::<&Path>::None
    }

    fn file_name(&self) -> impl AsRef<Path>;

    fn file_path(&self) -> PathBuf {
        Self::file_dir().map_or_else(
            || self.file_name().as_ref().to_path_buf(),
            |dir| dir.as_ref().join(self.file_name()),
        )
    }
}

impl Cacheable for OfficialItem {
    fn file_name(&self) -> impl AsRef<Path> {
        "item.json"
    }
}

impl Cacheable for RecipeItem {
    fn file_name(&self) -> impl AsRef<Path> {
        "recipe_item.json"
    }
}

impl Cacheable for ShopSummary {
    fn file_name(&self) -> impl AsRef<Path> {
        "shop_summary.json"
    }
}

impl Cacheable for People {
    fn file_name(&self) -> impl AsRef<Path> {
        "people.json"
    }
}

impl Cacheable for RequestReport {
    fn file_dir() -> Option<impl AsRef<Path>> {
        Some("request_report")
    }

    fn file_name(&self) -> impl AsRef<Path> {
        let arg = match self {
            RequestReport::All { date, hour } => format!("{}_{}h", date, hour),
            RequestReport::Shop { date, shop_id } => format!("{}_#{}", date, shop_id.0),
        };
        format!("request_report_{arg}.json")
    }
}

impl Cacheable for AreaSummary {
    fn file_name(&self) -> impl AsRef<Path> {
        "area_summary.json"
    }
}
