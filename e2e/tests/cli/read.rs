use assert_cmd::assert::OutputAssertExt;
use e2e::assert_json_snapshot;
use gw2fashionista_chatlink::fixtures::{self, ChatLinkFixture, all_fixtures, template_fixtures};
use rstest::rstest;
use rstest_reuse::{self, apply};

use e2e::{assert_snapshot, cli::spawn_cli};

const SNAPSHOT_DIR: &str = "read";

#[apply(all_fixtures)]
fn test_read_command(fixture: ChatLinkFixture) {
    let output = spawn_cli::<String>(&["read", fixture.link], None)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_snapshot!(&fixture.name, &output, SNAPSHOT_DIR);
}

#[apply(all_fixtures)]
fn test_read_command_pretty(fixture: ChatLinkFixture) {
    let output = spawn_cli::<String>(&["read", fixture.link, "--pretty"], None)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_snapshot!(&fixture.name, &output, SNAPSHOT_DIR);
}

#[apply(template_fixtures)]
fn test_read_command_skip_names(fixture: ChatLinkFixture) {
    let output = spawn_cli::<String>(&["read", fixture.link, "--skip-names"], None)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_snapshot!(&format!("skip_names_{}", fixture.name), &output, "read");
}

#[test]
fn test_read_command_input_list() {
    let templates = templates_as_list(fixtures::ALL_TEMPLATES);
    let input = templates.join("\n\n");
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success()
        .get_output()
        .clone();
    assert_all_templates(&output);
}

#[test]
fn test_read_command_input_list_invalid() {
    let templates = templates_as_list(fixtures::ALL_TEMPLATES);
    let input = format!("{}\nthis is not a chat link", templates.join("\n\n"));
    spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_read_command_input_list_invalid_lenient() {
    let templates = templates_as_list(fixtures::ALL_TEMPLATES);
    let input = format!("this is not a chat link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success()
        .get_output()
        .clone();
    assert_all_templates(&output);
}

#[test]
fn test_read_command_input_csv() {
    let templates = templates_as_csv(fixtures::ALL_TEMPLATES);
    let input = format!("name,fashion_link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success()
        .get_output()
        .clone();
    assert_all_templates(&output);
}

#[test]
fn test_read_command_input_csv_wrong_row() {
    let templates = templates_as_csv(fixtures::ALL_TEMPLATES);
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
    let templates = templates_as_csv(fixtures::ALL_TEMPLATES);
    let input = format!(
        "name,fashion_link\nwrong row,not a chat link\n{}",
        templates.join("\n\n")
    );
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success()
        .get_output()
        .clone();
    assert_all_templates(&output);
}

#[test]
fn test_read_command_input_csv_custom_column() {
    let templates = templates_as_csv(fixtures::ALL_TEMPLATES);
    let input = format!("name,link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .success()
        .get_output()
        .clone();
    assert_all_templates(&output);
}

#[test]
fn test_read_command_input_csv_column_missing() {
    let templates = templates_as_csv(fixtures::ALL_TEMPLATES);
    let input = format!("name,link_typo\n{}", templates.join("\n\n"));
    spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

fn assert_all_templates(output: &std::process::Output) {
    let stream =
        serde_json::Deserializer::from_slice(&output.stdout).into_iter::<serde_json::Value>();
    let json: Vec<_> = stream.collect::<Result<_, _>>().unwrap();
    assert_eq!(json.len(), fixtures::ALL_TEMPLATES.len());
    assert_json_snapshot!("all_templates", json, SNAPSHOT_DIR)
}

fn templates_as_csv(templates: &[ChatLinkFixture]) -> Vec<String> {
    templates
        .iter()
        .map(|t| format!("{},{}", t.name, t.link))
        .collect()
}

fn templates_as_list(templates: &[ChatLinkFixture]) -> Vec<String> {
    templates.iter().map(|t| t.link.to_string()).collect()
}
