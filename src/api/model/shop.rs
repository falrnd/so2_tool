use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UserId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(pub String);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}", self.0)
    }
}
