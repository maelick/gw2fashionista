use assert_cmd::Command;
use rstest::rstest;

use gw2fashionista_fixtures::wardrobe::{WardrobeTemplate, EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE};

fn spawn_cli(args: &[&str]) -> std::process::Output {
    Command::cargo_bin("gw2fashionista-cli")
        .expect("Failed to find cli binary")
        .args(args)
        .output()
        .expect("Failed to run command")
}

#[rstest]
#[case(EMPTY_TEMPLATE)]
#[case(PEEKABOO_TEMPLATE)]
#[case(ZIZI_TEMPLATE)]
#[case(ZIZI_ARMOR_TEMPLATE)]
fn test_read_command(#[case] template: WardrobeTemplate) {
    let output = spawn_cli(&["read", template.chat_link]);
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    insta::assert_json_snapshot!(template.snapshot_name("read"), json);
}

#[rstest]
#[case(EMPTY_TEMPLATE)]
#[case(PEEKABOO_TEMPLATE)]
#[case(ZIZI_TEMPLATE)]
#[case(ZIZI_ARMOR_TEMPLATE)]
fn test_read_command_pretty(#[case] template: WardrobeTemplate) {
    let output = spawn_cli(&["read", template.chat_link, "--pretty"]);
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    insta::assert_json_snapshot!(template.snapshot_name("read"), json);
}
