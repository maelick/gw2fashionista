use std::process::Output;

use assert_cmd::{Command, assert::OutputAssertExt};
use once_cell::sync::Lazy;
use regex::Regex;
use rstest::rstest;
use serde_json::Deserializer;

use gw2fashionista_fixtures::wardrobe::{
    ALL_TEMPLATES, EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, WardrobeTemplate, ZIZI_ARMOR_TEMPLATE,
    ZIZI_TEMPLATE,
};

const BASE64_RE: &str = r"[-A-Za-z0-9+/]*={0,3}";

static CHAT_LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(r"^\[?&?({})\]?$", BASE64_RE);
    Regex::new(&pattern).unwrap()
});

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]$").unwrap());

#[rstest]
#[case(EMPTY_TEMPLATE)]
#[case(PEEKABOO_TEMPLATE)]
#[case(ZIZI_TEMPLATE)]
#[case(ZIZI_ARMOR_TEMPLATE)]
fn test_read_command(#[case] template: WardrobeTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link], None)
        .assert()
        .success();
    assert_snapshot(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(EMPTY_TEMPLATE)]
#[case(PEEKABOO_TEMPLATE)]
#[case(ZIZI_TEMPLATE)]
#[case(ZIZI_ARMOR_TEMPLATE)]
fn test_read_command_pretty(#[case] template: WardrobeTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link, "--pretty"], None)
        .assert()
        .success();
    assert_snapshot(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(EMPTY_TEMPLATE)]
#[case(PEEKABOO_TEMPLATE)]
#[case(ZIZI_TEMPLATE)]
#[case(ZIZI_ARMOR_TEMPLATE)]
fn test_read_command_skip_names(#[case] template: WardrobeTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link, "--skip-names"], None)
        .assert()
        .success();
    assert_snapshot(
        output.get_output(),
        &template.snapshot_name("read_skip_names"),
    );
}

#[test]
fn test_read_command_input_list() {
    let templates = all_templates_as_list();
    let input = templates.join("\n\n");
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_list_invalid() {
    let templates = all_templates_as_list();
    let input = format!("{}\nthis is not a chat link", templates.join("\n\n"));
    spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_read_command_input_list_invalid_lenient() {
    let templates = all_templates_as_list();
    let input = format!("this is not a chat link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv() {
    let templates = all_templates_as_csv();
    let input = format!("name,fashion_link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_wrong_row() {
    let templates = all_templates_as_csv();
    let input = format!(
        "name,fashion_link\n{}\nwrong row,not a chat link",
        templates.join("\n\n")
    );
    spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_read_command_input_csv_wrong_row_lenient() {
    let templates = all_templates_as_csv();
    let input = format!(
        "name,fashion_link\nwrong row,not a chat link\n{}",
        templates.join("\n\n")
    );
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_custom_column() {
    let templates = all_templates_as_csv();
    let input = format!("name,link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_column_missing() {
    let templates = all_templates_as_csv();
    let input = format!("name,link_typo\n{}", templates.join("\n\n"));
    spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_export_command_csv() {
    fail_if_no_api_key();

    let output = spawn_cli::<String>(&["wardrobe", "export"], None)
        .assert()
        .success();
    let (headers, records) = read_csv(output.get_output());
    assert!(records.len() > 0);
    assert_eq!(headers.len(), 4);
    assert_eq!(headers.get(0).unwrap(), "char_name");
    assert_eq!(headers.get(1).unwrap(), "tab_id");
    assert_eq!(headers.get(2).unwrap(), "tab_name");
    assert_eq!(headers.get(3).unwrap(), "fashion_link");

    for record in records {
        assert_eq!(record.len(), 4);
        for field in record.iter() {
            assert_ne!(field, "");
        }

        assert!(
            NUMBER_REGEX.is_match(record.get(1).unwrap()),
            "second field should be a number"
        );
        assert!(
            CHAT_LINK_REGEX.is_match(record.get(3).unwrap()),
            "fourth field should be a chat link"
        );
    }
}

fn fail_if_no_api_key() {
    if api_key().is_none() {
        panic!("GW2_API_KEY not configured");
    };
}

fn api_key() -> Option<String> {
    std::env::var("GW2_API_KEY").ok()
}

fn spawn_cli<S>(args: &[&str], input: Option<S>) -> std::process::Output
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

fn all_templates_as_csv() -> Vec<String> {
    ALL_TEMPLATES
        .iter()
        .map(|t| format!("{},{}", t.name, t.chat_link))
        .collect()
}

fn all_templates_as_list() -> Vec<String> {
    ALL_TEMPLATES
        .iter()
        .map(|t| t.chat_link.to_string())
        .collect()
}

fn read_csv(output: &Output) -> (csv::StringRecord, Vec<csv::StringRecord>) {
    let output = output.stdout.clone();
    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(output));
    let records: Result<Vec<_>, _> = reader.records().collect();
    (reader.headers().unwrap().clone(), records.unwrap())
}

fn assert_snapshot(output: &Output, snapshot_name: &str) {
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    insta::assert_json_snapshot!(snapshot_name, json);
}

fn assert_all_templates(output: &Output) {
    let stream = Deserializer::from_slice(&output.stdout).into_iter::<serde_json::Value>();
    let json: Vec<_> = stream.collect::<Result<_, _>>().unwrap();
    assert_eq!(json.len(), ALL_TEMPLATES.len());
    insta::assert_json_snapshot!("read_input_list", json);
}
