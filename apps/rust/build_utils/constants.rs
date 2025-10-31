use once_cell::sync::Lazy;
use regex::Regex;

pub static HANDLER_FN_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap());

pub static ENDPOINT_METADATA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
      r#"const\s+(ENDPOINT_METHOD|ENDPOINT_PATH|ENDPOINT_DESCRIPTION|ENDPOINT_TAG|OPERATION_ID|SUCCESS_RESPONSE_BODY):\s*&\s*str\s*=\s*"([^"]*)";"#
    ).unwrap()
});

pub static DYNAMIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());
