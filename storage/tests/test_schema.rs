use std::time::Duration;

use sqlx::sqlite::SqlitePoolOptions;
use tokio::time::sleep;

#[tokio::test()]
async fn test_fashion_unique_constraints() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // Create the first fashion template
    sqlx::query!(
        "INSERT INTO fashion (id, name, wardrobe_template, travel_template) VALUES (?, ?, ?, ?)",
        "1",
        "test",
        "[&wardrobe]",
        "[&travel]"
    )
    .execute(&pool)
    .await
    .unwrap();

    // Create a fashion template with the same id (primary key) which should fail
    sqlx::query!(
        "INSERT INTO fashion (id, name, wardrobe_template, travel_template) VALUES (?, ?, ?, ?)",
        "1",
        "test2",
        "[&wardrobe]",
        "[&travel]"
    )
    .execute(&pool)
    .await
    .unwrap_err();

    // Create a second fashion template with the same name (and character name), which should fail
    sqlx::query!(
        "INSERT INTO fashion (id, name, wardrobe_template, travel_template) VALUES (?, ?, ?, ?)",
        "2",
        "test",
        "[&wardrobe]",
        "[&travel]"
    )
    .execute(&pool)
    .await
    .unwrap_err();

    // Create a second fashion template with the same name for a different character, which should succeed
    sqlx::query!(
        "INSERT INTO fashion (id, name, character, wardrobe_template, travel_template) VALUES (?, ?, ?, ?, ?)",
        "2",
        "test",
        "test_character",
        "[&wardrobe]",
        "[&travel]"
    )
    .execute(&pool)
    .await
    .unwrap();

    let fashion = sqlx::query!("SELECT COUNT(*) as count FROM fashion")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(fashion.count, 2);
}

#[tokio::test()]
async fn test_tag_unique_constraints() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // Create the first tag
    sqlx::query!("INSERT INTO tag (id, name) VALUES (?, ?)", "1", "test")
        .execute(&pool)
        .await
        .unwrap();

    // Create a tag with the same id, which should fail
    sqlx::query!("INSERT INTO tag (id, name) VALUES (?, ?)", "1", "test2")
        .execute(&pool)
        .await
        .unwrap_err();

    // Create a tag with the same name, which should fail
    sqlx::query!("INSERT INTO tag (id, name) VALUES (?, ?)", "2", "test")
        .execute(&pool)
        .await
        .unwrap_err();

    let tags = sqlx::query!("SELECT COUNT(*) as count FROM tag")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(tags.count, 1);
}

#[tokio::test()]
async fn test_character_update_timestamps() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    sqlx::query!(
        "INSERT INTO fashion (id, name, wardrobe_template, travel_template) VALUES (?, ?, ?, ?)",
        "1",
        "test",
        "[&wardrobe]",
        "[&travel]"
    )
    .execute(&pool)
    .await
    .unwrap();

    sleep(Duration::from_millis(100)).await;

    sqlx::query!(
        "UPDATE fashion SET description = ? WHERE id = ?",
        "Test",
        "1"
    )
    .execute(&pool)
    .await
    .unwrap();

    let fashion = sqlx::query!("SELECT * FROM fashion")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(fashion.len(), 1);
    assert!(fashion[0].updated_at > fashion[0].created_at);
}

#[tokio::test()]
async fn test_tag_update_timestamps() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    sqlx::query!("INSERT INTO tag (id, name) VALUES (?, ?)", "1", "test")
        .execute(&pool)
        .await
        .unwrap();

    sleep(Duration::from_millis(100)).await;

    sqlx::query!("UPDATE tag SET name = ? WHERE id = ?", "test2", "1")
        .execute(&pool)
        .await
        .unwrap();

    let tags = sqlx::query!("SELECT * FROM tag")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(tags.len(), 1);
    assert!(tags[0].updated_at > tags[0].created_at);
}
