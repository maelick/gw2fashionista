use gw2fashionista_core::domain::{chatlink::ChatLink, fashion::Fashion};
use gw2fashionista_fixtures::{travel, wardrobe};
use gw2fashionista_storage::repository::Repository;
use sqlx::SqlitePool;

#[sqlx::test]
async fn test_create_empty_fashion(pool: SqlitePool) {
    let repo = Repository::new(pool);

    // We create a new template
    let fashion = Fashion::new("empty_fashion".to_string(), None, None, None, None, &vec![]);
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
    let fashion = Fashion::new(
        "peekaboo".to_string(),
        Some("description".to_string()),
        Some("Pikku Peekaboo".to_string()),
        Some(
            ChatLink::from_string(wardrobe::PEEKABOO_TEMPLATE.chat_link)
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        Some(
            ChatLink::from_string(travel::PEEKABOO_TEMPLATE.chat_link)
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        &vec![],
    );
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
}
