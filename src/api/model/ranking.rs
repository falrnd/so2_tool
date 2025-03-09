use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{Position, area, shop};

pub static EPOCH: NaiveDate = NaiveDate::from_ymd_opt(2017, 5, 7).unwrap();

/// 月間全部門トップ3
#[derive(Debug, Clone, Deserialize)]
pub struct AllMonthly(pub HashMap<Category, Vec<Info>>);

/// 月間部門別トップ1000
#[derive(Debug, Clone, Deserialize)]
pub struct SectionMonthly(pub Vec<Info>);

/// デイリートップ1000
#[derive(Debug, Clone, Deserialize)]
pub struct Daily(pub Vec<DailyInfo>);

/// 部門名
// todo: リストを提供する
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    /// デイリーランキングトップ10入賞回数 (1-10位)
    pub top_10: Vec<u32>,
    /// トップ1獲得回数
    #[serde(default)] // actually nullable
    pub top1_total: u32,

    /// メダル数
    pub sort: u32,
    /// オーナー番号
    pub user_id: shop::UserId,
    /// ショップ番号
    pub shop_id: shop::Id,
    /// ショップ名
    pub shop_name: String,
    /// 街ID
    pub area_id: area::Id,
    /// キャッチコピー
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DailyInfo {
    /// 座標
    #[serde(flatten)]
    pub pos: Position,

    /// ポイントもしくは売上高メダル数
    pub point: u32,
    /// オーナー番号
    pub user_id: shop::UserId,
    /// ショップ番号
    pub shop_id: shop::Id,
    /// ショップ名
    pub shop_name: String,
    /// 街ID
    pub area_id: area::Id,
    /// キャッチコピー
    pub comment: String,
}
