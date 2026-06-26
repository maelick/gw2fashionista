#[cfg(test)]
pub mod cli;

pub fn normalize_json(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("Invalid JSON")
}
