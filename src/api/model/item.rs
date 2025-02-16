use std::collections::HashMap;

use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(flatten)]
    value: HashMap<String, Item>,
}

#[derive(Debug, Deserialize)]
pub struct Official(pub Response);

#[derive(Debug, Deserialize)]
pub struct Recipe(pub Response);

/// ## Note
/// ### 公式アイテムjsonの中に存在するレシピアイテム
/// - 公式アイテム"ミックスジュース"の素材?
/// - レシピアイテムjsonとの差異
///   - [Item::category] : 元のアイテムと同一 (全て"食物")
///   - [Item::class] : 元のアイテムと異なる, レシピの方が正 (全て"八百屋",公式は全て"食堂")
///   - [Item::item_id] : 元のアイテムと同一
///      - see also: [Response::join_table]
///   - [Item::limit] : 元のアイテムと異なる, レシピの方が正
///   - [Item::name] : "レシピ#元の名前"
///   - [Item::scale] : 元のアイテムと同一 (全て"個")
///   - [Item::sort] : 元のアイテムと異なる
#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub category: Category,
    pub class: Class,
    pub item_id: Id,
    pub limit: StackSize,
    pub name: Name,
    pub scale: Scale,
    pub sort: u32,
}

impl Item {
    pub fn is_official(&self) -> bool {
        self.item_id.is_official()
    }

    pub fn is_recipe(&self) -> bool {
        self.item_id.is_recipe()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Category(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Class(pub String);

///
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);

impl Id {
    pub fn is_official(&self) -> bool {
        self.0 < 2000000
    }

    pub fn is_recipe(&self) -> bool {
        !self.is_official()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackSize(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Scale(pub String);

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{}) ", self.name.0, self.category.0, self.class.0)
    }
}

impl std::hash::Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
    }
}

impl Response {
    pub fn values(&self) -> impl Iterator<Item = &Item> {
        self.value.values().sorted_by_key(|item| item.sort)
    }

    pub fn into_values(self) -> impl Iterator<Item = Item> {
        self.value.into_values().sorted_by_key(|item| item.sort)
    }

    pub fn join_table<S>(official: &Official, recipe: &Recipe) -> HashMap<Id, Item, S>
    where
        S: std::hash::BuildHasher + Default,
    {
        let o = official.0.values().filter(|v| v.is_official());
        let r = recipe.0.values(); //.filter(|v| v.is_recipe());

        o.chain(r)
            .map(|item| (item.item_id.clone(), item.clone()))
            .collect()
    }
}
