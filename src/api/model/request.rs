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
    pub seller_shop_id: shop::Id,
    pub seller_shop_name: shop::Name,
    pub buyer_shop_id: shop::Id,
    pub buyer_shop_name: shop::Name,
    pub item_id: item::Id,
    pub item_count: Amount,
    pub order_price: Price,
    pub traded_at: String, // NaiveDateTime?
}
