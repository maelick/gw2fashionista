use std::sync::LazyLock;

use assert_cmd::assert::OutputAssertExt;
use regex::Regex;

use e2e::{cli::spawn_cli, fail_if_no_api_key, read_csv};

static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]$").unwrap());

#[test]
fn test_export_command_csv() {
    fail_if_no_api_key();

    let output = spawn_cli::<String>(&["wardrobe", "export"], None)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let (headers, records) = read_csv(output);
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
            gw2fashionista_chatlink::CHAT_LINK_REGEX.is_match(record.get(3).unwrap()),
            "fourth field should be a chat link"
        );
    }
}
