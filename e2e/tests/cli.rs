use std::{process::Output, sync::LazyLock};

use assert_cmd::assert::OutputAssertExt;
use gw2fashionista_fixtures::FashionTemplate;
use regex::Regex;
use rstest::rstest;

use gw2fashionista_fixtures::wardrobe;

use e2e::{cli::spawn_cli, fail_if_no_api_key, read_csv};
use serde_json::Deserializer;

const BASE64_RE: &str = r"[-A-Za-z0-9+/]*={0,3}";

static CHAT_LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^\[?&?({})\]?$", BASE64_RE);
    Regex::new(&pattern).unwrap()
});

static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]$").unwrap());

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
fn test_read_command(#[case] template: FashionTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link], None)
        .assert()
        .success();
    assert_snapshot(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
fn test_read_command_pretty(#[case] template: FashionTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link, "--pretty"], None)
        .assert()
        .success();
    assert_snapshot(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
fn test_read_command_skip_names(#[case] template: FashionTemplate) {
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
    let templates = wardrobe::all_templates_as_list();
    let input = templates.join("\n\n");
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_list_invalid() {
    let templates = wardrobe::all_templates_as_list();
    let input = format!("{}\nthis is not a chat link", templates.join("\n\n"));
    spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_read_command_input_list_invalid_lenient() {
    let templates = wardrobe::all_templates_as_list();
    let input = format!("this is not a chat link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv() {
    let templates = wardrobe::all_templates_as_csv();
    let input = format!("name,fashion_link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_wrong_row() {
    let templates = wardrobe::all_templates_as_csv();
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
    let templates = wardrobe::all_templates_as_csv();
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
    let templates = wardrobe::all_templates_as_csv();
    let input = format!("name,link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_column_missing() {
    let templates = wardrobe::all_templates_as_csv();
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
    let (headers, records) = read_csv(output.get_output().stdout.clone());
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

fn assert_snapshot(output: &Output, snapshot_name: &str) {
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    insta::assert_json_snapshot!(snapshot_name, json);
}

fn assert_all_templates(output: &Output) {
    let stream = Deserializer::from_slice(&output.stdout).into_iter::<serde_json::Value>();
    let json: Vec<_> = stream.collect::<Result<_, _>>().unwrap();
    assert_eq!(json.len(), wardrobe::ALL_TEMPLATES.len());
    insta::assert_json_snapshot!("read_input_list", json);
}
