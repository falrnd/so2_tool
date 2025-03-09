use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, NaiveDateTime};
use serde::Deserialize;

use super::{item, shop};

#[derive(Debug, Deserialize)]
pub struct Response(pub Vec<RequestReport>);

/// 注文レポート
#[derive(Debug, Deserialize)]
pub struct RequestReport {
    // 配列で渡ってくるので順番を変えないように
    // 名前は変えてもいい
    /// 売却側ショップ番号
    pub seller_shop_id: shop::Id,
    /// 売却側ショップ名
    pub seller_shop_name: String,
    /// 注文側ショップ番号
    pub buyer_shop_id: shop::Id,
    /// 注文側ショップ名
    pub buyer_shop_name: String,
    /// 商品ID
    pub item_id: item::Id,
    /// 商品数量
    pub item_count: Amount,
    /// 注文単価
    pub order_price: Price,
    /// 取引時刻 (UNIX時間)
    pub unix_time: i64,
}

/// 商品数量
#[derive(Debug, Deserialize)]
pub struct Amount(pub u32);

/// 注文単価
#[derive(Debug, Deserialize)]
pub struct Price(pub u32);

impl RequestReport {
    pub fn traded_at(&self) -> NaiveDateTime {
        DateTime::from_timestamp(self.unix_time, 0)
            .unwrap()
            .naive_local()
    }
}

impl Display for RequestReport {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}({}) => {}({}) : ItemId({}) x{} @{} [{}]",
            self.seller_shop_name,
            self.seller_shop_id,
            self.buyer_shop_name,
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
