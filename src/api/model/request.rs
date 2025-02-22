use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

use super::{item, shop};

#[derive(Debug, Deserialize)]
pub struct RequestReport(pub Vec<Request>);

#[derive(Debug, Deserialize)]
pub struct Amount(pub u32);

#[derive(Debug, Deserialize)]
pub struct Price(pub u32);

#[derive(Debug, Deserialize)]
pub struct Request {
    // 配列で渡ってくるので順番を変えないように
    // 名前は変えてもいい
    pub seller_shop_id: shop::Id,
    pub seller_shop_name: shop::Name,
    pub buyer_shop_id: shop::Id,
    pub buyer_shop_name: shop::Name,
    pub item_id: item::Id,
    pub item_count: Amount,
    pub order_price: Price,
    pub traded_at: String, // NaiveDateTime?
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}({}), {}({}), ItemId({}) x{} @{} [{}]",
            self.seller_shop_name.0,
            self.seller_shop_id,
            self.buyer_shop_name.0,
            self.buyer_shop_id,
            self.item_id.0,
            self.item_count.0,
            self.order_price.0,
            self.traded_at
        )
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}G", self.0)
    }
}
