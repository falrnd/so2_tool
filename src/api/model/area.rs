// https://so2-api.mutoys.com/master/area.json

use std::collections::HashMap;

use serde::Deserialize;

use super::item;

pub type Response = HashMap<Id, Area>;

/// 街定義
#[derive(Debug, Clone, Deserialize)]
pub struct Area {
    /// 街ID
    pub area_id: Id,
    /// 街名
    pub name: Name,
    /// 街アピールコメント
    pub desc: Vec<String>,
    /// アピール商品ID
    #[serde(deserialize_with = "item::deserialize_optional_id")]
    pub icon: Option<item::Id>,
    /// マップX位置
    pub pos_x: u32,
    /// マップY位置
    pub pos_y: u32,
    /// マップ縦マス数
    pub height: u32,
    /// マップ横マス数
    pub width: u32,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);
