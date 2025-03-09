// https://so2-api.mutoys.com/master/area.json

use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use super::{Position, item};

pub type Response = HashMap<Id, Area>;

/// 街定義
#[derive(Debug, Clone, Deserialize)]
pub struct Area {
    /// 街ID
    pub area_id: Id,
    /// 街名
    pub name: String,
    /// 街アピールコメント
    pub desc: Vec<String>,
    /// アピール商品ID
    // "0"が渡って来うる(なぜ？)のでその場合はNone
    #[serde(deserialize_with = "item::deserialize_optional_id")]
    pub icon: Option<item::Id>,
    #[serde(flatten)]
    pub pos: Position,
    #[serde(flatten)]
    pub size: Size,
}

/// 街ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub NonZeroU8);

/// 街のマップマス数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    /// マップ縦マス数
    pub height: i32,
    /// マップ横マス数
    pub width: i32,
}

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

#[cfg(test)]
mod test {
    use super::Area;
    use serde_json::{Result, json};

    #[test]
    fn deserialize_area() -> Result<()> {
        let emerald = json!({
            "area_id": 1,
            "desc": [
              "水の都。王国最大の商業都市。",
              "知る人ぞ知る伝説の街です",
              "大都会エメラルド。地上に星の灯る街。"
            ],
            "height": 50,
            "icon": 174,
            "name": "エメラルド街",
            "pos_x": 130,
            "pos_y": 360,
            "width": 50
        });

        // icon item_id is 0
        let opal = json!({
            "area_id": 91,
            "desc": [
              "王国令:土地開発および他店への隣接を禁ず"
            ],
            "height": 50,
            "icon": 0,
            "name": "オパール街",
            "pos_x": 600,
            "pos_y": 800,
            "width": 50
        });

        serde_json::from_value::<Area>(emerald)?;
        serde_json::from_value::<Area>(opal)?;
        Ok(())
    }
}
