use gw2fashionista_core::domain::{chatlink::ChatLink, fashion::Fashion};
use gw2fashionista_fixtures::{travel, wardrobe};
use gw2fashionista_storage::repository::{Repository, StringFilters};
use sqlx::SqlitePool;

#[sqlx::test]
async fn test_create_empty_fashion(pool: SqlitePool) {
    let repo = Repository::new(pool);

    // We create a new template
    let fashion = Fashion::builder().name("empty_fashion").build();
    let created = &repo.insert_fashion(&fashion).await.unwrap();

    // Assert that timestamps and templates are set
    assert!(created.wardrobe_template.clone().unwrap().is_empty());
    assert!(created.travel_template.clone().unwrap().is_empty());
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We ensure we can't create it again
    repo.insert_fashion(&fashion).await.unwrap_err();

    // We retrieve the created template and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_fashion_by_id(created.id.unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created, &retrieved);

    let retrieved_by_name = repo
        .get_fashion_by_name("empty_fashion", None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created, &retrieved_by_name);

    let listed_fashions = repo.list_fashions().await.unwrap();
    assert_eq!(listed_fashions, vec![created.clone()]);
}

#[sqlx::test]
async fn test_create_not_empty_fashion(pool: SqlitePool) {
    let repo = Repository::new(pool);

    // We create a new template
    let fashion = Fashion::builder()
        .name("peekaboo")
        .description("description")
        .character("Pikku Peekaboo")
        .wardrobe_template(
            ChatLink::from_string(wardrobe::PEEKABOO_TEMPLATE.chat_link)
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .travel_template(
            ChatLink::from_string(travel::PEEKABOO_TEMPLATE.chat_link)
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .tags(bon::vec!["hello"])
        .build();
    let created = &repo.insert_fashion(&fashion).await.unwrap();

    // Assert that timestamps and templates are set
    assert!(!created.wardrobe_template.clone().unwrap().is_empty());
    assert!(!created.travel_template.clone().unwrap().is_empty());
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We ensure we can't create it again
    repo.insert_fashion(&fashion).await.unwrap_err();

    // We retrieve the created template and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_fashion_by_id(created.id.unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created, &retrieved);

    let retrieved_by_name = repo
        .get_fashion_by_name("peekaboo", Some("Pikku Peekaboo"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created, &retrieved_by_name);

    let listed_fashions = repo.list_fashions().await.unwrap();
    assert_eq!(listed_fashions, vec![created.clone()]);
}

#[sqlx::test]
async fn test_create_tag(pool: SqlitePool) {
    let repo = Repository::new(pool);

    // We create a new tag
    let created = &repo.ensure_tag("peekaboo").await.unwrap();

    // Assert that timestamps are set
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We create it again and ensure the returned tag is exactly the same
    let updated = repo.ensure_tag("peekaboo").await.unwrap();
    assert_eq!(created, &updated);

    // We retrieve the created tag and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_tag_by_id(&created.id.into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created, &retrieved);

    let retrieved_by_name = repo.get_tag_by_name("peekaboo").await.unwrap().unwrap();
    assert_eq!(created, &retrieved_by_name);

    let retrieved_tags = repo.list_tags(StringFilters::default()).await.unwrap();
    assert_eq!(retrieved_tags, vec![created.clone()]);
}

#[sqlx::test]
async fn test_list_tags(pool: SqlitePool) {
    let repo = Repository::new(pool);

    // We create tags
    let tag1 = &repo.ensure_tag("peekaboo").await.unwrap();
    let tag2 = &repo.ensure_tag("peekabo").await.unwrap();
    let tag3 = &repo.ensure_tag("aboo").await.unwrap();
    let tag4 = &repo.ensure_tag("peekboo").await.unwrap();

    // Assert empty filter returns all
    let retrieved_tags = repo.list_tags(StringFilters::default()).await.unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag1.clone(), tag2.clone(), tag3.clone(), tag4.clone()]
    );

    // Assert no match for prefix
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_prefix("hello"))
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert no match for suffix
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_suffix("hello"))
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert no match for substring
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_substrings(vec!["hello"]))
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert match for prefix
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_prefix("peek"))
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag2.clone(), tag1.clone(), tag4.clone()]
    );

    // Assert match for suffix
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_suffix("oo"))
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag1.clone(), tag3.clone(), tag4.clone()]
    );

    // Assert match for single substring
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_substrings(vec!["eek"]))
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag1.clone(), tag2.clone(), tag4.clone()]
    );

    // Assert match for multiple substrings
    let retrieved_tags = repo
        .list_tags(StringFilters::default().with_substrings(vec!["eek", "ka"]))
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone(), tag2.clone()]);

    // Assert match for prefix + substring
    let retrieved_tags = repo
        .list_tags(
            StringFilters::default()
                .with_prefix("peek")
                .with_suffix("oo"),
        )
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone(), tag4.clone()]);

    // Assert match for all
    let retrieved_tags = repo
        .list_tags(
            StringFilters::default()
                .with_prefix("peek")
                .with_suffix("oo")
                .with_substrings(vec!["ka"]),
        )
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone()]);
}
