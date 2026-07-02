pub mod cli;

pub fn fail_if_no_api_key() {
    if api_key().is_none() {
        panic!("GW2_API_KEY not configured");
    };
}

pub fn api_key() -> Option<String> {
    std::env::var("GW2_API_KEY").ok()
}
