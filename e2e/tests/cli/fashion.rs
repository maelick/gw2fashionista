use assert_cmd::assert::Assert;
use assert_cmd::assert::OutputAssertExt;
use e2e::assert_snapshot;
use rstest::fixture;
use rstest::rstest;

use gw2fashionista_fixtures::travel;
use gw2fashionista_fixtures::wardrobe;

use e2e::cli::spawn_cli;
use tempfile::TempPath;

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
            wardrobe::ZIZI_TEMPLATE.chat_link,
            "--travel",
            travel::ZIZI_TEMPLATE.chat_link,
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
    let cmd = list_fashion_cmd(&db_path, &[]);
    redact_fashion_list().bind(|| {
        assert_snapshot!(
            "test_fashion_list_empty_peekaboo_and_zizi",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

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
            wardrobe::PEEKABOO_TEMPLATE.chat_link,
            "--travel",
            travel::PEEKABOO_TEMPLATE.chat_link,
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
            wardrobe::PEEKABOO_TEMPLATE.chat_link,
            "--travel",
            travel::PEEKABOO_TEMPLATE.chat_link,
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
    let cmd = list_fashion_cmd(&db_path, &[]);
    redact_fashion_list().bind(|| {
        assert_snapshot!(
            "test_fashion_list_peekaboo_and_zizi",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });

    // Delete first template and list and asserts the templates
    fashion_cmd(&db_path, &["delete", "-n", "peekaboo"], None).success();
    let cmd = list_fashion_cmd(&db_path, &[]);
    redact_fashion_list().bind(|| {
        assert_snapshot!(
            "test_fashion_list_zizi",
            &cmd.success().get_output().stdout,
            SNAPSHOT_DIR
        )
    });
}

#[rstest]
fn test_fashion_chat_links(db_path: TempPath) {
    fashion_cmd(&db_path, &["create", "-n", "test_fashion"], None).success();

    // Wardrobe
    fashion_cmd(&db_path, &["wardrobe", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(wardrobe::EMPTY_TEMPLATE.wrap_chat_link() + "\n");
    fashion_cmd(
        &db_path,
        &["wardrobe", "set", "-n", "test_fashion"],
        Some(wardrobe::PEEKABOO_TEMPLATE.wrap_chat_link()),
    )
    .success();
    fashion_cmd(&db_path, &["wardrobe", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(wardrobe::PEEKABOO_TEMPLATE.wrap_chat_link() + "\n");

    // Travel
    fashion_cmd(&db_path, &["travel", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(travel::EMPTY_TEMPLATE.wrap_chat_link() + "\n");
    fashion_cmd(
        &db_path,
        &["travel", "set", "-n", "test_fashion"],
        Some(travel::PEEKABOO_TEMPLATE.wrap_chat_link()),
    )
    .success();
    fashion_cmd(&db_path, &["travel", "get", "-n", "test_fashion"], None)
        .success()
        .stdout(travel::PEEKABOO_TEMPLATE.wrap_chat_link() + "\n");
}

#[rstest]
fn test_fashion_tags(db_path: TempPath) {
    fashion_cmd(&db_path, &["create", "-n", "fashion1"], None).success();
    fashion_cmd(&db_path, &["create", "-n", "fashion2", "-t", "t1"], None).success();
    fashion_cmd(&db_path, &["tag", "list"], None).success();
    fashion_cmd(&db_path, &["list", "t1"], None).success();
    fashion_cmd(&db_path, &["list", "t2"], None).success();

    fashion_cmd(&db_path, &["patch", "-n", "fashion1", "-t", "t1"], None).success();
    fashion_cmd(&db_path, &["patch", "-n", "fashion2", "-t", "t2"], None).success();
    fashion_cmd(&db_path, &["tag", "list"], None).success();
    fashion_cmd(&db_path, &["list", "t1"], None).success();
    fashion_cmd(&db_path, &["list", "t2"], None).success();

    fashion_cmd(&db_path, &["set", "-n", "fashion2", "-t", "t2"], None).success();
    fashion_cmd(&db_path, &["tag", "list"], None).success();
    fashion_cmd(&db_path, &["list", "t1"], None).success();
    fashion_cmd(&db_path, &["list", "t2"], None).success();

    fashion_cmd(&db_path, &["untag", "-n", "fashion2"], None).success();
    fashion_cmd(&db_path, &["tag", "list"], None).success();
    fashion_cmd(&db_path, &["list", "t1"], None).success();
    fashion_cmd(&db_path, &["list", "t2"], None).success();

    fashion_cmd(&db_path, &["tag", "clean"], None).success();
    fashion_cmd(&db_path, &["tag", "list"], None).success();
    fashion_cmd(&db_path, &["list", "t1"], None).success();
    fashion_cmd(&db_path, &["list", "t2"], None).success();
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
