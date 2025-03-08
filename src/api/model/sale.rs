use serde::{Deserialize, Serialize};

use super::{item, shop};

pub type Response = Vec<Sale>;

#[derive(Debug, Clone, Serialize, Deserialize)]
///販売品
pub struct Sale {
    /// 販売通し番号
    pub sale_serial: i64,
    /// 街ID
    pub area_id: i64,
    /// X座標
    pub pos_x: i64,
    /// Y座標
    pub pos_y: i64,
    /// オーナー番号
    pub user_id: shop::UserId,
    /// ショップ番号
    pub shop_id: shop::Id,
    /// ショップ名
    pub shop_name: shop::Name,
    /// 商品ID
    pub item_id: item::Id,
    /// 販売単価
    pub price: i64,
    /// 販売在庫数
    pub unit: i64,
    /// まとめ売り
    #[serde(with = "super::serde_bool_int")]
    pub bundle_sale: bool,
}
