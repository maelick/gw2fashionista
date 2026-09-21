#[macro_export]
macro_rules! assert_snapshot {
    ($json:expr, $slice:expr) => {
        $crate::assert_snapshot!($outpsliceut, $json, None)
    };
    ($snapshot_name:expr, $slice:expr, $subdir:expr) => {{
        let json: serde_json::Value = serde_json::from_slice($slice).unwrap();
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
        $crate::snapshot::settings($subdir)
            .bind(|| insta::assert_json_snapshot!(snapshot_name, $json));
    }};
}

pub fn settings<'a>(subdir: impl Into<Option<&'a str>>) -> insta::Settings {
    let subdir: Option<&str> = subdir.into();
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    let subdir = subdir.unwrap_or_default();
    if !subdir.is_empty() {
        settings.set_snapshot_path(format!("snapshots/{subdir}"));
    }
    settings
}
