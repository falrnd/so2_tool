// https://so2-api.mutoys.com/master/area.json

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);
