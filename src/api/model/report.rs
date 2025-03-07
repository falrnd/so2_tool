use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{area, item};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// 住民の購入レポート
    pub system: ReportItem,
    /// 業者(店頭)の購入レポート
    pub user: ReportItem,
    /// 業者(注文)の購入レポート
    pub request: ReportItem,
    // 町別レポート
    pub area: HashMap<area::Id, Report>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// 住民の購入レポート
    pub system: ReportItem,
    /// 業者(店頭)の購入レポート
    pub user: ReportItem,
    /// 業者(注文)の購入レポート
    pub request: ReportItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportItem {
    pub item: HashMap<item::Id, ReportEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportEntry {
    /// 総取引件数
    pub count: u64,
    /// 総取引数量
    pub unit: u64,
    /// 総取引額
    pub money: u64,
    /// 取引単価
    pub price: u64,
}
