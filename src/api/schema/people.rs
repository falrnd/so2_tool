use std::collections::HashMap;
use std::time::Duration;

use itertools::Itertools;
use serde::Deserialize;

use crate::api::APICall;

use super::{area, Schema};

const FILE_NAME: &str = r"people.json";

pub struct Quely {}

impl Schema for Quely {
    type Response = Response;

    fn endpoint(&self) -> url::Url {
        super::ORIGIN.join("json/people/all.json").unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Response(Vec<PeopleOfTown>);

#[derive(Debug, Deserialize)]
pub struct PeopleOfTown {
    pub area_id: area::Id,
    pub unit: Population,
    /// key: "1", "2", ...
    pub persons: HashMap<String, Person>,
    pub trend: Option<Vec<Trend>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Person {
    pub unit: Population,
    pub name: Name,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trend {
    pub area_id: area::Id,
    #[serde(rename = "isPositive")]
    pub is_positive: bool,
    pub status: TrendStatusString,
    pub message: TrendMessage,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Population(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct TrendStatusString(pub String);

/// -5 ~ +5 (maybe)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrendStatus(pub i8);

#[derive(Debug, Clone, Deserialize)]
pub struct TrendMessage(pub String);

impl Response {
    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {
        APICall::new(Quely {}.endpoint(), FILE_NAME)
            .set_interval(Duration::from_secs(600))
            .load_cache_or_call()
            .await
    }

    pub fn values(&self) -> impl Iterator<Item = &PeopleOfTown> {
        self.0.iter()
    }
}

impl std::fmt::Display for PeopleOfTown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AreaInfo {{ Area{:?}, {:?}, people: [{}], trend: [{}] }}",
            self.area_id,
            self.unit,
            self.persons
                .iter()
                .sorted_by_cached_key(|v| v.0.parse::<i32>().unwrap())
                .map(|v| v.1)
                .join(", "),
            self.trend.iter().flatten().join(", "),
        )
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name.0, self.unit.0)
    }
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Trend {{ Area{:?}, {} ({}) }}",
            self.area_id, self.message.0, self.status.0,
        )
    }
}
