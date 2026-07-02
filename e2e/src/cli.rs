use std::process::Output;

use assert_cmd::Command;
use serde_json::Deserializer;

use gw2fashionista_fixtures::wardrobe::ALL_TEMPLATES;

use crate::api_key;

pub fn spawn_cli<S>(args: &[&str], input: Option<S>) -> std::process::Output
where
    S: Into<Vec<u8>>,
{
    let mut cmd = Command::cargo_bin("gw2fashionista-cli").expect("Failed to find cli binary");
    let cmd = cmd.args(args);
    if let Some(input) = input {
        cmd.write_stdin(input);
    }
    if let Some(api_key) = api_key() {
        cmd.env("GW2_API_KEY", api_key);
    }
    cmd.output().expect("Failed to run command")
}

pub fn read_csv(output: &Output) -> (csv::StringRecord, Vec<csv::StringRecord>) {
    let output = output.stdout.clone();
    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(output));
    let records: Result<Vec<_>, _> = reader.records().collect();
    (reader.headers().unwrap().clone(), records.unwrap())
}

pub fn assert_snapshot(output: &Output, snapshot_name: &str) {
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    insta::assert_json_snapshot!(snapshot_name, json);
}

pub fn assert_all_templates(output: &Output) {
    let stream = Deserializer::from_slice(&output.stdout).into_iter::<serde_json::Value>();
    let json: Vec<_> = stream.collect::<Result<_, _>>().unwrap();
    assert_eq!(json.len(), ALL_TEMPLATES.len());
    insta::assert_json_snapshot!("read_input_list", json);
}
