use assert_cmd::Command;

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
