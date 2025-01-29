use std::sync::LazyLock;

use regex::Regex;

pub static URL_SAFE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_]+$").expect("Unable to create regex"));
pub static TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[!#$%&'*+\-\.\^_`\|\~0-9a-zA-Z]+$").expect("Unable to create regex")
});
pub static MALICIOUS_STRS: [&'static str; 3] = ["\n", "\r", "\\"];
