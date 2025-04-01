use askama::Result;
use serde_json::Value;

pub fn json_encode(value: &Value) -> Result<String> {
    match serde_json::to_string_pretty(value) {
        Ok(s) => Ok(s),
        Err(e) => Err(askama::Error::Custom(Box::new(e))),
    }
}
