use serde::{Deserialize, Serialize};

use super::{area, item, shop};

pub type Response = Vec<Request>;

/// 注文品
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// 注文通し番号
    trans_serial: i64,
    /// 街ID
    area_id: area::Id,
    /// オーナー番号
    user_id: shop::UserId,
    /// ショップ番号
    shop_id: shop::Id,
    /// ショップ名
    shop_name: String,
    /// 商品ID
    item_id: item::Id,
    /// 買い取り済み数量
    unit: i32,
    /// 買い付け希望数
    buy_unit: i32,
    /// 注文単価
    price: i32,
    /// 注文対象範囲
    // doc: 0:全域対象注文 1～:範囲対象街のID
    // impl: 0:None
    #[serde(with = "area::serde_id_opt")]
    request_area_id: Option<area::Id>,
}
