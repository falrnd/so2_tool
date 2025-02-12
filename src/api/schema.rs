use std::sync::LazyLock;
use std::time::Duration;

use url::Url;

const DEFAULT_INTERVAL: Duration = Duration::from_secs(3600);

pub static ORIGIN: LazyLock<url::Url> =
    LazyLock::new(|| url::Url::parse("https://so2-api.mutoys.com").unwrap());

pub trait Schema {
    type Response: for<'de> serde::de::Deserialize<'de>;

    fn endpoint(&self) -> Url;

    fn min_interval(&self) -> Duration {
        DEFAULT_INTERVAL
    }
}
