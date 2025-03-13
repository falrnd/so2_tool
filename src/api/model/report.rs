use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::area;

/// レポート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    // 全体レポート
    #[serde(flatten)]
    pub general: AreaReport,
    // 町別レポート
    pub area: HashMap<area::Id, AreaReport>,
}

/// レポート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaReport {
    /// 住民の購入レポート
    pub system: Entries,
    /// 業者(店頭)の購入レポート
    pub user: Entries,
    /// 業者(注文)の購入レポート
    pub request: Entries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entries {
    // pub item: HashMap<item::Id, Entry>,
    pub item: HashMap<String, Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// 総取引件数
    pub count: u64,
    /// 総取引数量
    pub unit: u64,
    /// 総取引額
    pub money: u64,
    /// 取引単価
    pub price: u64,
}
