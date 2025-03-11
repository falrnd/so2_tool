use std::collections::HashMap;

use itertools::Itertools;
use serde::Deserialize;
use serde_with::{DefaultOnNull, serde_as};

use super::area;

/// 全街人口リスト
pub type Response = Vec<People>;

/// 街の人口リスト
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct People {
    /// 街ID
    pub area_id: area::Id,
    /// 街人口
    pub unit: Population,
    /// 住民別人口
    /// key: "1", "2", ...
    pub persons: HashMap<String, Segment>,
    /// 流行情報
    // 流行情報が無い時は "trend":null
    #[serde_as(as = "DefaultOnNull")]
    pub trend: Vec<Trend>,
}

/// 住民別人口
#[derive(Debug, Clone, Deserialize)]
pub struct Segment {
    /// 人口
    pub unit: Population,
    /// 住民名
    pub name: SegmentType,
}

/// 流行情報
#[derive(Debug, Clone, Deserialize)]
pub struct Trend {
    /// 街ID
    pub area_id: area::Id,
    /// ムード
    #[serde(rename = "isPositive")]
    pub is_positive: bool,
    /// 状態
    pub status: TrendStatusString,
    /// コメント
    pub message: TrendMessage,
}

/// 人口
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Population(pub u32);

/// 住民名
#[derive(Debug, Clone, Deserialize)]
pub struct SegmentType(pub String);

/// 流行情報
/// * ex) "↑↑↑"
#[derive(Debug, Clone, Deserialize)]
pub struct TrendStatusString(pub String);

/// -5 ~ +5 (maybe)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrendStatus(pub i8);

/// コメント
#[derive(Debug, Clone, Deserialize)]
pub struct TrendMessage(pub String);

impl std::fmt::Display for People {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AreaInfo {{ Area{:?}, {:?}, people: [{}], trend: [{}] }}",
            self.area_id,
            self.unit,
            self.persons
                .iter()
                .sorted_by_cached_key(|v| v.0.parse::<i32>().unwrap())
                .map(|v| v.1)
                .join(", "),
            self.trend.iter().join(", "),
        )
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name.0, self.unit.0)
    }
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Trend {{ Area{:?}, {} ({}) }}",
            self.area_id, self.message.0, self.status.0,
        )
    }
}
