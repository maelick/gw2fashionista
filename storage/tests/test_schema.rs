use sqlx::SqlitePool;

#[sqlx::test]
async fn test_fashion_unique_constraints(pool: SqlitePool) {
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

#[sqlx::test]
async fn test_tag_unique_constraints(pool: SqlitePool) {
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
