use std::str::FromStr;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::traits::Validate;

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn to_url(&self) -> &str {
        &self.url
    }
}

impl Validate for URL {
    fn validate(&self) -> anyhow::Result<()> {
        Self::is_valid_url(&self.url)
    }
}

#[cfg(test)]
mod tests {}
