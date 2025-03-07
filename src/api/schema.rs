use std::{borrow::Cow, sync::LazyLock, time::Duration};

use chrono::{Datelike, NaiveDate};
use url::Url;

use super::model::*;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(3600);

pub trait Schema {
    type Response: for<'de> serde::Deserialize<'de>;

    fn endpoint(&self) -> Url;

    fn min_interval() -> Duration {
        DEFAULT_INTERVAL
    }

    fn formatter(&self) -> Formatter {
        Formatter::default()
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
    pub ym: NaiveDate,
}
///ランキング/月間部門別トップ1000
pub struct RankingSectionMonthly {
    pub ym: NaiveDate,
    pub section: String,
}
///ランキング/デイリートップ1000
pub struct RankingSectionDaily {
    pub date: NaiveDate,
    pub section: String,
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

impl_schema! {
    OfficialItem => item::Official { ORIGIN.join("master/item.json").unwrap() }
    RecipeItem => item::Recipe { ORIGIN.join("json/master/recipe_item.json").unwrap() }
    Area => area::Response { ORIGIN.join("master/area.json").unwrap() }
    Report => report::Response { |self|
        let yyyy = self.0.year();
        let mm = self.0.month();
        let dd = self.0.day();
        ORIGIN.join(&format!("json/report/buy{yyyy:04}{mm:02}{dd:02}.json")).unwrap()
    }
    RankingAllMonthly => ranking::AllMonthly { |self|
        let yyyy = self.ym.year();
        let mm = self.ym.month();
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}/summary.json")).unwrap()
    }
    RankingSectionMonthly => ranking::SectionMonthly { |self|
        let yyyy = self.ym.year();
        let mm = self.ym.month();
        let section = &self.section;
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}/{section}.json")).unwrap()
    }
    RankingSectionDaily => ranking::Daily { |self|
        let yyyy = self.date.year();
        let mm = self.date.month();
        let dd = self.date.day();
        let section = &self.section;
        ORIGIN.join(&format!("json/ranking/{yyyy:04}-{mm:02}-{dd:02}/{section}.json")).unwrap()
    }
    Sale => sale::Response { ORIGIN.join("json/sale/all.json").unwrap() } {
        fn min_interval() -> Duration {
            Duration::from_secs(600)
        }
    }
    Request => request::Response { ORIGIN.join("json/request/all.json").unwrap() } {
        fn min_interval() -> Duration {
            Duration::from_secs(600)
        }
    }
    ShopSummary => shop_summary::ShopSummary { ORIGIN.join("json/shop/summary.json").unwrap() }
    Shop => shop::Response { ORIGIN.join("json/shop/all.json").unwrap() }
    People => people::Response { ORIGIN.join("json/people/all.json").unwrap() } {
        fn min_interval() -> Duration {
            Duration::from_secs(600)
        }
    }
    RequestReport => request_report::Response { |self|
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
        fn formatter(&self) -> Formatter {
            Formatter::Special
        }
    }
    AreaSummary => area_summary::Response { ORIGIN.join("json/area/summary.json").unwrap() }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Formatter {
    #[default]
    None,
    Special,
}

impl Formatter {
    pub fn format<'a>(&self, bytes: &'a [u8]) -> Cow<'a, [u8]> {
        match self {
            Formatter::None => Cow::Borrowed(bytes),
            Formatter::Special => {
                let mut vec = Vec::with_capacity(bytes.len() + 2);
                vec.push(b'[');
                vec.extend_from_slice(bytes);
                if vec.last() == Some(&b',') {
                    vec.pop();
                }
                vec.push(b']');
                Cow::Owned(vec)
            }
        }
    }
}
