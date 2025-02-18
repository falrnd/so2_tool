use std::fmt::{Display, Formatter, Result};

use itertools::Itertools;
use serde::Deserialize;

use super::area;

#[derive(Debug, Deserialize)]
pub struct ShopSummary {
    pub total: ShopCount,
    pub areas: Vec<AreaShopSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShopCount(pub u32);

impl Display for ShopCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} 店舗", self.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AreaShopSummary {
    pub area_id: area::Id, // APIドキュメントだとstringになっているが?
    pub name: area::Name,
    pub count: ShopCount,
}

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
        write!(f, "{} ({})", self.name.0, self.count)
    }
}
