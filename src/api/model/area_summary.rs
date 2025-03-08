use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

use super::area;

#[derive(Debug, Deserialize)]
pub struct Response(pub Vec<AreaSummary>);

#[derive(Debug, Deserialize)]
pub struct AreaSummary {
    pub area_id: area::Id,
    pub point: Fun,
}

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
