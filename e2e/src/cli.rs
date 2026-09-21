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

#[macro_export]
macro_rules! assert_snapshot {
    ($output:expr, $json:expr) => {
        $crate::assert_snapshot!($output, $json, None)
    };
    ($output:expr, $snapshot_name:expr, $subdir:expr) => {{
        let json: serde_json::Value = serde_json::from_slice(&$output.stdout).unwrap();
        $crate::assert_json_snapshot!($snapshot_name, json, $subdir)
    }};
}

#[macro_export]
macro_rules! assert_json_snapshot {
    ($snapshot_name:expr, $json:expr) => {
        $crate::assert_json_snapshot!($snapshot_name, $json, None)
    };
    ($snapshot_name:expr, $json:expr, $subdir:expr) => {{
        let snapshot_name: &str = $snapshot_name;
        $crate::cli::insta_settings($subdir)
            .bind(|| insta::assert_json_snapshot!(snapshot_name, $json));
    }};
}

pub fn insta_settings(subdir: Option<&str>) -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    let subdir = subdir.unwrap_or_default();
    if !subdir.is_empty() {
        settings.set_snapshot_path(format!("snapshots/{subdir}"));
    }
    settings
}
