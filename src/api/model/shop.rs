use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

use super::area;

pub type Response = Vec<Shop>;

/// ショップ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    /// オーナー番号
    pub user_id: UserId,
    /// ショップ番号
    pub shop_id: Id,
    /// ショップ名
    pub shop_name: String,
    /// キャッチコピー
    #[serde(default)]
    pub comment: Option<String>,
    /// 街ID
    pub area_id: area::Id,
    /// X座標
    pub pos_x: i32,
    /// Y座標
    pub pos_y: i32,
    /// お店種類
    pub shop_type: String,
    /// 資金
    pub money: i32,
    /// 称号
    pub title: String,
    /// ポイント
    pub point: i32,
    /// 創業日数
    pub foundation_days: i32,
    /// 元祖創業日数 (SO1データ引き継ぎをしていない場合は値自体が存在しません)
    #[serde(default)]
    pub so1_foundation_days: Option<i32>,
    /// 商品図鑑登録数
    pub item_book: i32,
    /// 称号業種IDと称号業種レベル
    // (該当データが無い場合は[0,0])
    pub high_class: TitleClass,
    /// 称号職種IDと称号職種レベル
    // (該当データが無い場合は[0,0])
    pub high_job: TitleJob,
}

/// ショップ番号
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Id(pub u32);

/// オーナー番号
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UserId(pub u32);

/// 称号業種
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TitleClass {
    /// 称号業種ID
    pub id: ClassId,
    /// 称号業種レベル
    pub level: Level,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TitleJob {
    /// 称号職種ID
    pub id: JobId,
    /// 秱号職種レベル
    pub level: Level,
}

/// 業種ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClassId(pub u32);

/// 職種ID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct JobId(pub u32);

/// 業種/職種レベル
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(pub u32);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}", self.0)
    }
}
