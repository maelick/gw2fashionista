use assert_cmd::assert::Assert;
use assert_cmd::assert::OutputAssertExt;
use e2e::assert_snapshot;
use gw2fashionista_chatlink::fixtures;
use gw2fashionista_chatlink::fixtures::{travel, wardrobe};
use rstest::fixture;
use rstest::rstest;
use serde::Deserialize;
use serde::Serialize;
use tempfile::TempPath;

use e2e::cli::spawn_cli;

const SNAPSHOT_DIR: &str = "fashion";

#[fixture]
fn db_path() -> TempPath {
    tempfile::Builder::new()
        .prefix("gw2fashionsta-e2e-")
        .suffix(".sqlite")
        .tempfile()
        .unwrap()
        .into_temp_path()
}

#[rstest]
fn test_fashion(db_path: TempPath) {
    // DB should be empty
    fashion_cmd(&db_path, &["list"], None)
        .success()
        .stdout("[]");
    fashion_cmd(&db_path, &["tag", "list"], None)
        .success()
        .stdout("[]");

    // Create an empty template
    let cmd = fashion_cmd(&db_path, &["create", "-n", "peekaboo"], None);
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_empty_peekaboo",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // Create the same template
    fashion_cmd(&db_path, &["create", "-n", "peekaboo"], None).failure();

    // Create another template with data
    let cmd = fashion_cmd(
        &db_path,
        &[
            "create",
            "-n",
            "zizi",
            "-d",
            "This is a description",
            "--wardrobe",
            wardrobe::ZIZI_TEMPLATE.link,
            "--travel",
            travel::ZIZI_TEMPLATE.link,
            "-t",
            "tag1,tag2",
        ],
        None,
    );
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_zizi",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // List and assert the created templates
    assert_fashion_list(&db_path, "test_fashion_list_empty_peekaboo_and_zizi");

    // Patch the first template with data
    let cmd = fashion_cmd(
        &db_path,
        &[
            "patch",
            "-n",
            "peekaboo",
            "-d",
            "Another description",
            "--wardrobe",
            wardrobe::PEEKABOO_TEMPLATE.link,
            "--travel",
            travel::PEEKABOO_TEMPLATE.link,
            "-t",
            "tag1,tag2",
        ],
        None,
    );
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_peekaboo",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // Patch the first template with empty data
    let cmd = fashion_cmd(&db_path, &["patch", "-n", "peekaboo"], None);
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_peekaboo",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // Update the first template with empty data
    let cmd = fashion_cmd(&db_path, &["set", "-n", "peekaboo"], None);
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_empty_peekaboo",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // Update the first template with data
    let cmd = fashion_cmd(
        &db_path,
        &[
            "set",
            "-n",
            "peekaboo",
            "-d",
            "Another description",
            "--wardrobe",
            wardrobe::PEEKABOO_TEMPLATE.link,
            "--travel",
            travel::PEEKABOO_TEMPLATE.link,
            "-t",
            "tag1,tag2",
        ],
        None,
    );
    redact_fashion().bind(|| {
        assert_snapshot!(
            "test_fashion_peekaboo",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // List and assert the updated templates
    assert_fashion_list(&db_path, "test_fashion_list_peekaboo_and_zizi");

    // Delete first template and list and asserts the templates
    fashion_cmd(&db_path, &["delete", "-n", "peekaboo"], None).success();
    assert_fashion_list(&db_path, "test_fashion_list_zizi");
}

#[rstest]
fn test_fashion_chat_links(db_path: TempPath) {
    fashion_cmd(&db_path, &["create", "-n", "test_fashion"], None).success();

    // Wardrobe
    fashion_cmd(&db_path, &["wardrobe", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(wardrobe::EMPTY_TEMPLATE.wrap() + "\n");
    fashion_cmd(
        &db_path,
        &["wardrobe", "set", "-n", "test_fashion"],
        Some(wardrobe::PEEKABOO_TEMPLATE.wrap()),
    )
    .success();
    fashion_cmd(&db_path, &["wardrobe", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(wardrobe::PEEKABOO_TEMPLATE.wrap() + "\n");

    // Travel
    fashion_cmd(&db_path, &["travel", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(travel::EMPTY_TEMPLATE.wrap() + "\n");
    fashion_cmd(
        &db_path,
        &["travel", "set", "-n", "test_fashion"],
        Some(travel::PEEKABOO_TEMPLATE.wrap()),
    )
    .success();
    fashion_cmd(&db_path, &["travel", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(travel::PEEKABOO_TEMPLATE.wrap() + "\n");
}

#[rstest]
fn test_fashion_tags(db_path: TempPath) {
    // Creae two templates, the second one with a tag
    fashion_cmd(&db_path, &["create", "-n", "fashion1"], None).success();
    fashion_cmd(&db_path, &["create", "-n", "fashion2", "-t", "tag1"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Add the same tag to the first template, and a second tag to the second template
    fashion_cmd(&db_path, &["patch", "-n", "fashion1", "-t", "tag1"], None).success();
    fashion_cmd(&db_path, &["patch", "-n", "fashion2", "-t", "tag2"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag2"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1", "fashion2"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag2"]).success()),
        ["fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Rename the second tag to a new tag
    fashion_cmd(&db_path, &["tag", "rename", "tag2", "tag3"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag3"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1", "fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag3"]).success()),
        ["fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Replace the renamed tag with a new tag
    fashion_cmd(&db_path, &["tag", "replace", "tag3", "tag4"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag3", "tag4"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1", "fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag4"]).success()),
        ["fashion2"]
    );

    // Delete the renamed tag
    fashion_cmd(&db_path, &["tag", "delete", "tag4"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag3"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1", "fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Replace all tags on the second template with the original (recreated)
    fashion_cmd(&db_path, &["set", "-n", "fashion2", "-t", "tag2"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag2", "tag3"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag2"]).success()),
        ["fashion2"]
    );
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Untag the second template
    fashion_cmd(&db_path, &["untag", "-n", "fashion2"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1", "tag2", "tag3"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");

    // Clean unused tags
    fashion_cmd(&db_path, &["tag", "clean"], None).success();
    assert_eq!(
        names(fashion_cmd(&db_path, &["tag", "list", "-f", "json"], None).success()),
        ["tag1"]
    );
    assert_eq!(
        names(list_fashion_cmd(&db_path, &["tag1"]).success()),
        ["fashion1"]
    );
    list_fashion_cmd(&db_path, &["tag2"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag3"]).success().stdout("[]");
    list_fashion_cmd(&db_path, &["tag4"]).success().stdout("[]");
}

#[rstest]
fn test_fashion_import(db_path: TempPath) {
    let import_data = vec![
        ExportedEquipment {
            name: "Equipment tab 1".to_string(),
            character: "Character One".to_string(),
            tab_id: 1,
            wardrobe_template: fixtures::wardrobe::ZIZI_TEMPLATE.link.to_string(),
        },
        ExportedEquipment {
            name: "Equipment tab 2".to_string(),
            character: "Character One".to_string(),
            tab_id: 2,
            wardrobe_template: fixtures::wardrobe::ZIZI_ARMOR_TEMPLATE.link.to_string(),
        },
        ExportedEquipment {
            name: "Equipment tab 1".to_string(),
            character: "Character Two".to_string(),
            tab_id: 1,
            wardrobe_template: fixtures::wardrobe::ZIZI_TEMPLATE.link.to_string(),
        },
        ExportedEquipment {
            name: "Equipment tab 1".to_string(),
            character: "Character Three".to_string(),
            tab_id: 1,
            wardrobe_template: fixtures::wardrobe::ZIZI_ARMOR_TEMPLATE.link.to_string(),
        },
    ];

    fashion_cmd(
        &db_path,
        &[
            "create",
            "-n",
            "Equipment tab 1",
            "-c",
            "Character One",
            "-d",
            "The description",
            "--wardrobe",
            fixtures::wardrobe::PEEKABOO_TEMPLATE.link,
        ],
        None,
    )
    .success();
    assert_fashion_list(&db_path, "test_fashion_import_01_noimport");

    import_fashions(&db_path, "skip", &import_data).failure();
    let fashions_with_skipped = assert_fashion_list(&db_path, "test_fashion_import_02_after_skip");

    import_fashions(&db_path, "fail", &import_data).failure();
    assert_eq!(
        fashions_with_skipped.get_output().stdout,
        assert_fashion_list(&db_path, "test_fashion_import_02_after_skip")
            .get_output()
            .stdout
    );

    fashion_cmd(
        &db_path,
        &[
            "patch",
            "-n",
            "Equipment tab 1",
            "-c",
            "Character Two",
            "-d",
            "The other description",
            "-t",
            "tag",
            "--travel",
            fixtures::travel::KABOOM_TEMPLATE.link,
        ],
        None,
    )
    .success();
    assert_fashion_list(&db_path, "test_fashion_import_03_after_manual_patch");

    import_fashions(&db_path, "patch", &import_data).success();
    assert_fashion_list(&db_path, "test_fashion_import_04_after_patch");

    import_fashions(&db_path, "rename", &import_data).success();
    assert_fashion_list(&db_path, "test_fashion_import_05_after_rename");

    import_fashions(&db_path, "rename", &import_data).success();
    assert_fashion_list(&db_path, "test_fashion_import_06_after_second_rename");

    import_fashions(&db_path, "overwrite", &import_data).success();
    assert_fashion_list(&db_path, "test_fashion_import_07_after_overwrite");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportedEquipment {
    pub name: String,
    pub character: String,
    pub tab_id: usize,
    pub wardrobe_template: String,
}

fn import_fashions(db_path: &TempPath, mode: &str, import_data: &[ExportedEquipment]) -> Assert {
    fashion_cmd(
        db_path,
        &["import", "-m", mode, "--json"],
        Some(serde_json::to_string(import_data).unwrap()),
    )
}

fn assert_fashion_list(db_path: &TempPath, snapshot_name: &str) -> Assert {
    let cmd = list_fashion_cmd(db_path, &[]);
    let assert = cmd.success();
    redact_fashion_list()
        .bind(|| assert_snapshot!(snapshot_name, &assert.get_output().stdout, SNAPSHOT_DIR));
    assert
}

fn list_fashion_cmd(db_path: &TempPath, tags: &[&str]) -> Assert {
    let args = [&["list"], tags].concat();
    fashion_cmd(db_path, &args, None)
}

fn fashion_cmd(db_path: &TempPath, args: &[&str], input: Option<String>) -> Assert {
    let args = [&["--db", db_path.to_str().unwrap(), "fashion"], args].concat();
    spawn_cli(&args, input).assert()
}

fn redact_fashion() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.add_redaction(".id", "[uuid]");
    settings.add_redaction(".created_at", "[datetime]");
    settings.add_redaction(".updated_at", "[datetime]");
    settings
}

fn redact_fashion_list() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.add_redaction("[].id", "[uuid]");
    settings.add_redaction("[].created_at", "[datetime]");
    settings.add_redaction("[].updated_at", "[datetime]");
    settings
}

#[derive(serde::Deserialize)]
struct Named {
    name: String,
}

fn names(assert: Assert) -> Vec<String> {
    let items: Vec<Named> = serde_json::from_slice(&assert.get_output().stdout).unwrap();
    items.into_iter().map(|item| item.name).collect()
}
