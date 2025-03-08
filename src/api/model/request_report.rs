use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, NaiveDateTime};
use serde::Deserialize;

use super::{item, shop};

#[derive(Debug, Deserialize)]
pub struct Response(pub Vec<RequestReport>);

#[derive(Debug, Deserialize)]
pub struct Amount(pub u32);

#[derive(Debug, Deserialize)]
pub struct Price(pub u32);

#[derive(Debug, Deserialize)]
pub struct RequestReport {
    // 配列で渡ってくるので順番を変えないように
    // 名前は変えてもいい
    pub seller_shop_id: shop::Id,
    pub seller_shop_name: shop::Name,
    pub buyer_shop_id: shop::Id,
    pub buyer_shop_name: shop::Name,
    pub item_id: item::Id,
    pub item_count: Amount,
    pub order_price: Price,
    pub timestamp: i64,
}

impl RequestReport {
    pub fn traded_at(&self) -> NaiveDateTime {
        DateTime::from_timestamp(self.timestamp, 0)
            .unwrap()
            .naive_local()
    }
}

impl Display for RequestReport {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}({}) => {}({}) : ItemId({}) x{} @{} [{}]",
            self.seller_shop_name.0,
            self.seller_shop_id,
            self.buyer_shop_name.0,
            self.buyer_shop_id,
            self.item_id.0,
            self.item_count.0,
            self.order_price,
            self.traded_at()
        )
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}G", self.0)
    }
}
