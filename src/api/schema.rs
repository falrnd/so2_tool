use std::{sync::LazyLock, time::Duration};

use chrono::{Datelike, NaiveDate};
use url::Url;

use super::model::*;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(3600);

pub trait Schema {
    type Response: for<'de> serde::Deserialize<'de>;

    fn endpoint(&self) -> Url;

    fn min_interval(&self) -> Duration {
        DEFAULT_INTERVAL
    }

    // for 特殊フォーマット
    fn parse(&self, bytes: &[u8]) -> Result<Self::Response, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

macro_rules! impl_endpoint {
    (|$self:ident| $($ep:tt)+) => { fn endpoint($self:&Self) -> Url { $($ep)+ } };
    ($($ep:stmt)+) => { impl_endpoint!{ |self| $($ep)+ } };
}

macro_rules! impl_schema {
    ( $(
        $self:ty => $res:ty { $($ep:tt)+ } $({ $($other_impls:tt)* })?
    )+ ) => {
        $(
            impl Schema for $self {
                type Response = $res;
                impl_endpoint!{ $($ep)+ }
                $( $($other_impls)* )?
            }
        )+
    };
}

///商品定義
pub struct OfficialItem;
///レシピ商品定義
pub struct RecipeItem;
///街定義
pub struct Area;
//.レポート
pub struct Report(pub NaiveDate);
///ランキング/月間全部門トップ3
pub struct RankingAllMonthly {
    ym: NaiveDate,
}
///ランキング/月間部門別トップ1000
pub struct RankingSectionMonthly {
    ym: NaiveDate,
    section: String,
}
///ランキング/デイリートップ1000
pub struct RankingSectionDaily {
    date: NaiveDate,
    section: String,
}
///販売品
pub struct Sale;
///注文品
pub struct Request;
///お店件数
pub struct ShopSummary;
///全お店リスト
pub struct Shop;
//.住民
pub struct People;
///注文レポート
pub enum RequestReport {
    ///全注文
    All { date: NaiveDate, hour: u8 },
    ///ショップ別注文
    Shop { date: NaiveDate, shop_id: shop::Id },
}
///街情報
pub struct AreaSummary;

static ORIGIN: LazyLock<Url> = LazyLock::new(|| Url::parse("https://so2-api.mutoys.com").unwrap());

#[derive(serde::Deserialize)]
pub struct _UNIMPLEMENTED;

impl_schema! {
    OfficialItem => item::Official { ORIGIN.join("master/item.json").unwrap() }
    RecipeItem => item::Recipe { ORIGIN.join("json/master/recipe_item.json").unwrap() }
    Area => _UNIMPLEMENTED { ORIGIN.join("master/area.json").unwrap() }
    Report => _UNIMPLEMENTED { |self|
        let yyyy = self.0.year();
        let mm = self.0.month();
        let dd = self.0.day();
        ORIGIN.join(&format!("json/report/buy{yyyy:04}{mm:02}{dd:02}")).unwrap()
    }
    RankingAllMonthly => _UNIMPLEMENTED { |self|
        let yyyy = self.ym.year();
        let mm = self.ym.month();
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}/all.json")).unwrap()
    }
    RankingSectionMonthly => _UNIMPLEMENTED { |self|
        let yyyy = self.ym.year();
        let mm = self.ym.month();
        let section = &self.section;
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}/{section}.json")).unwrap()
    }
    RankingSectionDaily => _UNIMPLEMENTED { |self|
        let yyyy = self.date.year();
        let mm = self.date.month();
        let dd = self.date.day();
        let section = &self.section;
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}-{dd:02}/{section}.json")).unwrap()
    }
    Sale => _UNIMPLEMENTED { ORIGIN.join("json/sale/all.json").unwrap() } {
        fn min_interval(&self) -> Duration {
            Duration::from_secs(600)
        }
    }
    Request => _UNIMPLEMENTED { ORIGIN.join("json/request/all.json").unwrap() } {
        fn min_interval(&self) -> Duration {
            Duration::from_secs(600)
        }
    }
    ShopSummary => shop_summary::ShopSummary { ORIGIN.join("json/shop/summary.json").unwrap() }
    Shop => _UNIMPLEMENTED { ORIGIN.join("json/shop/all.json").unwrap() }
    People => people::Response { ORIGIN.join("json/people/all.json").unwrap() } {
        fn min_interval(&self) -> Duration {
            Duration::from_secs(600)
        }
    }
    RequestReport => _UNIMPLEMENTED { |self|
        let (date, arg) = match self {
            RequestReport::All { date, hour } => {
                (date, format!("all/{hour:02}"))
            },
            RequestReport::Shop {date, shop_id } => {
                (date, format!("shop/{}", shop_id.0))
            },
        };
        let yyyy = date.year();
        let mm = date.month();
        let dd = date.day();
        ORIGIN.join(&format!("json/request/{yyyy:04}/{mm:02}/{dd:02}/{arg}.json")).unwrap()
    }
    {
        fn parse(&self, _bytes: &[u8]) -> Result<Self::Response, serde_json::Error> {
            todo!()
        }
    }
    AreaSummary => area_summary::Response { ORIGIN.join("json/area/summary.json").unwrap() }
}
