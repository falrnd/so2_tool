use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

use super::area;

pub type Response = Vec<AreaSummary>;

/// 街情報
#[derive(Debug, Deserialize)]
pub struct AreaSummary {
    /// 街ID
    pub area_id: area::Id,
    /// 活気ポイント
    pub point: Fun,
}

/// 活気ポイント
#[derive(Debug, Deserialize)]
pub struct Fun(pub i32);

impl Display for AreaSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(AreaId({}), {})", self.area_id.0, self.point)
    }
}

impl Display for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.0)
    }
}
