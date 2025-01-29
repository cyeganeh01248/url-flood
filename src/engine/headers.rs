use std::str::FromStr;

use anyhow::{anyhow, Error, Ok};
use serde::{Deserialize, Serialize};

use super::{
    statics::{MALICIOUS_STRS, URL_SAFE_REGEX},
    traits::Validate,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    key: String,
    val: String,
}

impl FromStr for Header {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let n1 = split.next();
        let n2 = split.next();

        if n1.is_none() {
            return Err(anyhow!("No key provided."));
        }
        if n2.is_none() {
            return Err(anyhow!("No val provided."));
        }
        let key = n1.unwrap().to_owned();
        let val = n2.unwrap().to_owned();

        Header::new(key, val)
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.val)
    }
}
impl Header {
    pub fn new(key: String, val: String) -> Result<Self, anyhow::Error> {
        let header = Self { key, val };
        header.validate()?;
        Ok(header)
    }
    pub fn is_valid_key(key: &str) -> Result<(), anyhow::Error> {
        if URL_SAFE_REGEX.is_match(key) {
            Ok(())
        } else {
            Err(anyhow!("Invalid or empty key provided. ({key})"))
        }
    }
    pub fn is_valid_val(val: &str) -> anyhow::Result<()> {
        for malicious_str in MALICIOUS_STRS {
            if val.contains(malicious_str) {
                return Err(anyhow!("Invalid or malicious key provided. ({val})"));
            }
        }
        Ok(())
    }
}

impl Validate for Header {
    fn validate(&self) -> anyhow::Result<()> {
        Self::is_valid_key(&self.key).and(Self::is_valid_val(&self.val))
    }
}
