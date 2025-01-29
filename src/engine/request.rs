use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{cookies::Cookie, headers::Header, traits::Validate, url::URL};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EngineRequest {
    pub url: URL,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub body: String,
}

impl EngineRequest {
    pub fn new(
        url: String,
        mut headers: Vec<Header>,
        cookies: Vec<Cookie>,
        body: Option<impl ToString>,
    ) -> EngineRequest {
        let body = match body {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        headers.push(Header::new("Content-Length".to_owned(), body.len().to_string()).unwrap());
        EngineRequest {
            url: URL::from_str(&url).unwrap(),
            cookies,
            body,
            headers,
        }
    }
}

impl Validate for EngineRequest {
    fn validate(&self) -> Result<(), anyhow::Error> {
        let mut result = self.url.validate();
        for header in self.headers.iter() {
            result = result.and(header.validate());
        }
        for cookie in self.cookies.iter() {
            result = result.and(cookie.validate())
        }
        result
    }
}
