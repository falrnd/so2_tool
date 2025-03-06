// https://so2-api.mutoys.com/master/area.json

use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub NonZeroU8);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);

pub mod serde_id_opt {
    use std::num::NonZeroU8;

    use super::Id;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(id: &Option<Id>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        id.as_ref().map_or(0, |id| id.0.get()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Id>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(NonZeroU8::new(Deserialize::deserialize(deserializer)?).map(Id))
    }
}
