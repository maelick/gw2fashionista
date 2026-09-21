use assert_cmd::assert::OutputAssertExt;
use e2e::assert_json_snapshot;
use gw2fashionista_fixtures::FashionTemplate;
use gw2fashionista_fixtures::templates_as_csv;
use gw2fashionista_fixtures::templates_as_list;
use rstest::rstest;

use gw2fashionista_fixtures::travel;
use gw2fashionista_fixtures::wardrobe;

use e2e::{assert_snapshot, cli::spawn_cli};

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
#[case(travel::EMPTY_TEMPLATE)]
#[case(travel::DEFAULT_MOUNT_TEMPLATE)]
#[case(travel::NO_DYES_TEMPLATE)]
#[case(travel::PEEKABOO_TEMPLATE)]
#[case(travel::KABOOM_TEMPLATE)]
#[case(travel::KABOOM_MOUNTS_TEMPLATE)]
#[case(travel::ZIZI_TEMPLATE)]
fn test_read_command(#[case] template: FashionTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link], None)
        .assert()
        .success();
    assert_snapshot!(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
#[case(travel::EMPTY_TEMPLATE)]
#[case(travel::DEFAULT_MOUNT_TEMPLATE)]
#[case(travel::NO_DYES_TEMPLATE)]
#[case(travel::PEEKABOO_TEMPLATE)]
#[case(travel::KABOOM_TEMPLATE)]
#[case(travel::KABOOM_MOUNTS_TEMPLATE)]
#[case(travel::ZIZI_TEMPLATE)]
fn test_read_command_pretty(#[case] template: FashionTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link, "--pretty"], None)
        .assert()
        .success();
    assert_snapshot!(output.get_output(), &template.snapshot_name("read"));
}

#[rstest]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
#[case(travel::EMPTY_TEMPLATE)]
#[case(travel::DEFAULT_MOUNT_TEMPLATE)]
#[case(travel::NO_DYES_TEMPLATE)]
#[case(travel::PEEKABOO_TEMPLATE)]
#[case(travel::KABOOM_TEMPLATE)]
#[case(travel::KABOOM_MOUNTS_TEMPLATE)]
#[case(travel::ZIZI_TEMPLATE)]
fn test_read_command_skip_names(#[case] template: FashionTemplate) {
    let output = spawn_cli::<String>(&["read", template.chat_link, "--skip-names"], None)
        .assert()
        .success();
    assert_snapshot!(
        output.get_output(),
        &template.snapshot_name("read_skip_names")
    );
}

fn all_templates() -> Vec<FashionTemplate> {
    [wardrobe::ALL_TEMPLATES, travel::ALL_TEMPLATES].concat()
}

#[test]
fn test_read_command_input_list() {
    let templates = templates_as_list(&all_templates());
    let input = templates.join("\n\n");
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_list_invalid() {
    let templates = templates_as_list(&all_templates());
    let input = format!("{}\nthis is not a chat link", templates.join("\n\n"));
    spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .failure()
        .stdout("");
}

#[test]
fn test_read_command_input_list_invalid_lenient() {
    let templates = templates_as_list(&all_templates());
    let input = format!("this is not a chat link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "--lenient"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv() {
    let templates = templates_as_csv(&all_templates());
    let input = format!("name,fashion_link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_wrong_row() {
    let templates = templates_as_csv(&all_templates());
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
    let templates = templates_as_csv(&all_templates());
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
    let templates = templates_as_csv(&all_templates());
    let input = format!("name,link\n{}", templates.join("\n\n"));
    let output = spawn_cli::<String>(&["read", "-c", "link"], Some(input))
        .assert()
        .success();
    assert_all_templates(output.get_output());
}

#[test]
fn test_read_command_input_csv_column_missing() {
    let templates = templates_as_csv(&all_templates());
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
    assert_eq!(
        json.len(),
        travel::ALL_TEMPLATES.len() + wardrobe::ALL_TEMPLATES.len()
    );
    assert_json_snapshot!("read_input_list", json)
}
