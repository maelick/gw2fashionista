use assert_cmd::Command;

use gw2fashionista_fixtures::wardrobe::{EMPTY_TEMPLATE};

fn spawn_cli(args: &[&str]) -> std::process::Output {
    Command::cargo_bin("gw2fashionista-cli")
        .expect("Failed to find cli binary")
        .args(args)
        .output()
        .expect("Failed to run command")
}

#[test]
fn test_read_command() {
    let output = spawn_cli(&["read", EMPTY_TEMPLATE]);
    assert!(output.status.success());
}
