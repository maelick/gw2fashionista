pub mod cli;

pub fn fail_if_no_api_key() {
    if api_key().is_none() {
        panic!("GW2_API_KEY not configured");
    };
}

pub fn api_key() -> Option<String> {
    std::env::var("GW2_API_KEY").ok()
}

pub fn read_csv(output: Vec<u8>) -> (csv::StringRecord, Vec<csv::StringRecord>) {
    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(output));
    let records: Result<Vec<_>, _> = reader.records().collect();
    (reader.headers().unwrap().clone(), records.unwrap())
}
