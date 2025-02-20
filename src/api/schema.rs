use std::{sync::LazyLock, time::Duration};

use chrono::NaiveDate;
use url::Url;

use super::model::*;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(3600);

pub trait Schema {
    type Response: for<'de> serde::de::Deserialize<'de>;

    fn endpoint(&self) -> Url;

    fn min_interval(&self) -> Duration {
        DEFAULT_INTERVAL
    }

    // for 特殊フォーマット
    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

macro_rules! impl_schema {
    ( $(
        $self:ident => $res:path { $($ep:stmt)+ } $({ $($other_impls:tt)* })?
    )+ ) => {
        $(
            impl Schema for $self {
                type Response = $res;
                fn endpoint(&self) -> Url { $($ep)+ }
                $( $($other_impls)* )?
            }
        )+
    };
}

pub struct OfficialItem;
pub struct RecipeItem;
pub struct Area;
pub struct Report(pub NaiveDate);
pub enum Ranking {
    AllMonthly { ym: NaiveDate },
    SectionMonthly { ym: NaiveDate, section: String },
    SectionDaily { date: NaiveDate, section: String },
}
pub struct Sale;
pub struct Request;
pub struct ShopSummary;
pub struct Shop;
pub struct People;
pub enum RequestReport {
    All { hour: u8 },
    Shop { shop_id: shop::Id },
}
pub struct AreaSummary;

static ORIGIN: LazyLock<Url> = LazyLock::new(|| Url::parse("https://so2-api.mutoys.com").unwrap());

impl_schema! {
    OfficialItem => item::Official { ORIGIN.join("master/item.json").unwrap() }
    RecipeItem => item::Recipe { ORIGIN.join("json/master/recipe_item.json").unwrap() }
    ShopSummary => shop_summary::ShopSummary { ORIGIN.join("json/shop/summary.json").unwrap() }
    People => people::Response { ORIGIN.join("json/people/all.json").unwrap() } {
        fn min_interval(&self) -> Duration {
            Duration::from_secs(600)
        }
    }
    AreaSummary => area_summary::Response { ORIGIN.join("json/area/summary.json").unwrap() }
}
