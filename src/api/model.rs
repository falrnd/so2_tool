pub mod area;
pub mod area_summary;
pub mod item;
pub mod people;
pub mod ranking;
pub mod request;
pub mod request_report;
pub mod sale;
pub mod shop;
pub mod shop_summary;

mod serde_bool_int {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let i: u8 = if *b { 1 } else { 0 };
        i.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = u8::deserialize(deserializer)?;
        Ok(i != 0)
    }
}
