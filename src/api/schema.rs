use std::{sync::LazyLock, time::Duration};

use url::Url;

use super::model::{item, people};

const DEFAULT_INTERVAL: Duration = Duration::from_secs(3600);

pub static ORIGIN: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://so2-api.mutoys.com").unwrap());

pub trait Schema {
    type Response: for<'de> serde::de::Deserialize<'de>;

    fn endpoint(&self) -> Url;

    fn min_interval(&self) -> Duration {
        DEFAULT_INTERVAL
    }
}

impl Schema for item::Request {
    type Response = item::Response;

    fn endpoint(&self) -> Url {
        ORIGIN.join("master/item.json").unwrap()
    }
}

impl Schema for people::Request {
    type Response = people::Response;

    fn endpoint(&self) -> Url {
        ORIGIN.join("json/people/all.json").unwrap()
    }

    fn min_interval(&self) -> Duration {
        Duration::from_secs(600)
    }
}
