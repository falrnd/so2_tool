use serde::{Deserialize, Serialize};

pub mod area;
pub mod area_summary;
pub mod item;
pub mod people;
pub mod ranking;
pub mod report;
pub mod request;
pub mod request_report;
pub mod sale;
pub mod shop;
pub mod shop_summary;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// マップX位置
    pub pos_x: i32,
    /// マップY位置
    pub pos_y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.pos_x, self.pos_y)
    }
}
