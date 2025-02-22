use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);
