use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Deserialize)]
pub struct Name(pub String);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}", self.0)
    }
}
