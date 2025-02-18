// https://so2-api.mutoys.com/master/area.json

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);
