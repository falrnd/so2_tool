use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;

use super::{Position, area, shop};

pub static EPOCH: NaiveDate = NaiveDate::from_ymd_opt(2017, 5, 7).unwrap();

#[derive(Debug, Clone, Deserialize)]
pub struct AllMonthly(pub HashMap<Category, Vec<Info>>);

#[derive(Debug, Clone, Deserialize)]
pub struct SectionMonthly(pub Vec<Info>);

#[derive(Debug, Clone, Deserialize)]
pub struct Daily(pub Vec<DailyInfo>);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub top_10: Vec<u32>,
    #[serde(default)] // actually nullable
    pub top1_total: u32,

    pub sort: u32,
    #[serde(default)]
    pub user_id: Option<shop::UserId>,
    #[serde(default)]
    pub shop_id: Option<shop::Id>,
    pub shop_name: String,
    pub area_id: area::Id,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DailyInfo {
    #[serde(flatten)]
    pub pos: Position,

    pub point: u32,
    #[serde(default)]
    pub user_id: Option<shop::UserId>,
    #[serde(default)]
    pub shop_id: Option<shop::Id>,
    pub shop_name: String,
    pub area_id: area::Id,
    pub comment: String,
}
