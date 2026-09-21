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
    ($output:expr, $snapshot_name:expr) => {{
        let json: serde_json::Value = serde_json::from_slice(&$output.stdout).unwrap();
        $crate::assert_json_snapshot!($snapshot_name, json)
    }};
}

#[macro_export]
macro_rules! assert_all_templates {
    ($output:expr) => {{
        let stream =
            serde_json::Deserializer::from_slice(&$output.stdout).into_iter::<serde_json::Value>();
        let json: Vec<_> = stream.collect::<Result<_, _>>().unwrap();
        assert_eq!(
            json.len(),
            travel::ALL_TEMPLATES.len() + wardrobe::ALL_TEMPLATES.len()
        );
        $crate::assert_json_snapshot!("read_input_list", json)
    }};
}

#[macro_export]
macro_rules! assert_json_snapshot {
    ($snapshot_name:expr, $json:expr) => {{
        let snapshot_name: &str = $snapshot_name;
        $crate::cli::insta_settings().bind(|| insta::assert_json_snapshot!(snapshot_name, $json));
    }};
}

pub fn insta_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings
}
