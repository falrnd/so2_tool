use std::fmt::{Display, Formatter, Result};

use itertools::Itertools;
use serde::Deserialize;

use super::area;

/// お店件数
#[derive(Debug, Deserialize)]
pub struct ShopSummary {
    /// 全店舗数
    pub total: ShopCount,
    /// 街別店舗数
    pub areas: Vec<AreaShopSummary>,
}

/// 街別店舗数
#[derive(Debug, Clone, Deserialize)]
pub struct AreaShopSummary {
    /// エリアID
    pub area_id: area::Id, // APIドキュメントだとstringになっている
    /// 街名
    pub name: String,
    /// 店舗数
    pub count: ShopCount,
}

/// 店舗数
#[derive(Debug, Clone, Deserialize)]
pub struct ShopCount(pub u32);

impl Display for ShopSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ShopSummary {{\n\ttotal: {},\n\tareas: [\n\t\t{}\n\t]\n}}",
            self.total,
            self.areas.iter().join(",\n\t\t")
        )
    }
}

impl Display for AreaShopSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.name, self.count)
    }
}

impl Display for ShopCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} 店舗", self.0)
    }
}
