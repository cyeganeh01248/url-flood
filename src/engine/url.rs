use std::str::FromStr;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::traits::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct URL {
    url: String,
}

impl FromStr for URL {
    type Err = Error;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            url: url.to_string(),
        })
    }
}
impl URL {
    pub fn new(url: String) -> anyhow::Result<Self> {
        Self::is_valid_url(&url)?;
        Ok(Self { url })
    }
    pub fn is_valid_url(_url: &str) -> anyhow::Result<()> {
        // TODO
        // URL_REGEX.is_match(url)
        Ok(())
    }
}

impl Validate for URL {
    fn validate(&self) -> anyhow::Result<()> {
        Self::is_valid_url(&self.url)
    }
}
