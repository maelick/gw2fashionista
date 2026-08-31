use gw2fashionista_core::domain::{chatlink::ChatLink, fashion::Fashion};
use gw2fashionista_fixtures::{travel, wardrobe};
use gw2fashionista_storage::{Repository, StringFilters, sqlite};
use sqlx::{SqlitePool, types::uuid};

#[sqlx::test]
async fn test_create_empty_fashion(pool: SqlitePool) {
    let repo = sqlite::Repository::new(pool);

    // We create a new template
    let fashion = Fashion::builder().name("empty_fashion").build();
    let created = &repo.insert_fashion(&fashion).await.unwrap();

    // Assert that timestamps are set and fields are not
    assert!(created.description.is_none());
    assert!(created.character.is_none());
    assert!(created.wardrobe_template.clone().unwrap().is_empty());
    assert!(created.travel_template.clone().unwrap().is_empty());
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We ensure we can't create it again
    repo.insert_fashion(&fashion).await.unwrap_err();

    // We retrieve the created template and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_fashion_by_id(&created.id.unwrap())
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
    let repo = sqlite::Repository::new(pool);

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

    // Assert that timestamps and fields are set
    assert_eq!(created.description.clone().unwrap(), "description");
    assert_eq!(created.character.clone().unwrap(), "Pikku Peekaboo");
    assert!(!created.wardrobe_template.clone().unwrap().is_empty());
    assert!(!created.travel_template.clone().unwrap().is_empty());
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We ensure we can't create it again
    repo.insert_fashion(&fashion).await.unwrap_err();

    // We retrieve the created template and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_fashion_by_id(&created.id.unwrap())
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
async fn test_update_fashion(pool: SqlitePool) {
    let repo = sqlite::Repository::new(pool);

    // We create a new empty template
    let fashion = Fashion::builder().name("peekaboo").build();
    let created = &repo.insert_fashion(&fashion).await.unwrap();

    // We update it with fields
    let fashion = Fashion::builder()
        .id(created.id.unwrap())
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
    let updated = &repo.update_fashion(&fashion).await.unwrap().unwrap();

    // Assert that timestamps are set and fields are not
    assert_eq!(updated.description.clone().unwrap(), "description");
    assert_eq!(updated.character.clone().unwrap(), "Pikku Peekaboo");
    assert!(!updated.wardrobe_template.clone().unwrap().is_empty());
    assert!(!updated.travel_template.clone().unwrap().is_empty());
    assert!(updated.updated_at.unwrap() > updated.created_at.unwrap());

    // We retrieve the updated template and ensure it is identical to the one returned by the update.
    let retrieved = repo
        .get_fashion_by_id(&updated.id.unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated, &retrieved);

    let retrieved_by_name = repo
        .get_fashion_by_name("peekaboo", Some("Pikku Peekaboo"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated, &retrieved_by_name);

    let listed_fashions = repo.list_fashions().await.unwrap();
    assert_eq!(listed_fashions, vec![updated.clone()]);

    // We update it back to empty
    let fashion = Fashion::builder()
        .id(created.id.unwrap())
        .name("peekaboo")
        .build();
    let updated = &repo.update_fashion(&fashion).await.unwrap().unwrap();

    // Assert that timestamps are set and fields are not
    assert!(created.description.is_none());
    assert!(created.character.is_none());
    assert!(created.wardrobe_template.clone().unwrap().is_empty());
    assert!(created.travel_template.clone().unwrap().is_empty());
    assert!(updated.updated_at.unwrap() > updated.created_at.unwrap());

    // We update a template that does not exist
    let fashion = Fashion::builder()
        .id(uuid::Uuid::now_v7())
        .name("does not exist")
        .build();
    let updated = repo.update_fashion(&fashion).await.unwrap();
    assert!(updated.is_none())
}

#[sqlx::test]
async fn test_create_tag(pool: SqlitePool) {
    let repo = sqlite::Repository::new(pool);

    // We create a new tag
    let created = &repo.ensure_tag("peekaboo").await.unwrap();

    // Assert that timestamps are set
    assert_eq!(created.created_at.unwrap(), created.updated_at.unwrap());

    // We create it again and ensure the returned tag is exactly the same
    let updated = repo.ensure_tag("peekaboo").await.unwrap();
    assert_eq!(created, &updated);

    // We retrieve the created tag and ensure it is identical to the one returned by insertion.
    let retrieved = repo
        .get_tag_by_id(&created.id.unwrap())
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
    let repo = sqlite::Repository::new(pool);

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
        .list_tags(StringFilters::builder().prefix("hello").build())
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert no match for suffix
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().suffix("hello").build())
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert no match for substring
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().substrings(["hello"]).build())
        .await
        .unwrap();
    assert!(retrieved_tags.is_empty());

    // Assert match for prefix
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().prefix("peek").build())
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag2.clone(), tag1.clone(), tag4.clone()]
    );

    // Assert match for suffix
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().suffix("oo").build())
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag1.clone(), tag3.clone(), tag4.clone()]
    );

    // Assert match for single substring
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().substrings(["eek"]).build())
        .await
        .unwrap();
    assert_eq!(
        retrieved_tags,
        vec![tag1.clone(), tag2.clone(), tag4.clone()]
    );

    // Assert match for multiple substrings
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().substrings(["eek", "ka"]).build())
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone(), tag2.clone()]);

    // Assert match for prefix + substring
    let retrieved_tags = repo
        .list_tags(StringFilters::builder().prefix("peek").suffix("oo").build())
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone(), tag4.clone()]);

    // Assert match for all
    let retrieved_tags = repo
        .list_tags(
            StringFilters::builder()
                .prefix("peek")
                .suffix("oo")
                .substrings(["ka"])
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(retrieved_tags, vec![tag1.clone()]);
}

#[sqlx::test]
async fn test_crud_fashion_tags(pool: SqlitePool) {
    let repo = sqlite::Repository::new(pool);

    // We create two templates
    let fashion1 = &repo
        .insert_fashion(&Fashion::builder().name("fashion1").build())
        .await
        .unwrap();
    let fashion2 = &repo
        .insert_fashion(&Fashion::builder().name("fashion2").build())
        .await
        .unwrap();

    // We create 2 tags
    repo.upsert_tag("tag1").await.unwrap();
    repo.upsert_tag("tag2").await.unwrap();

    // We ensure the templates have no tags
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert!(tags.is_empty());
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert!(tags.is_empty());

    // We add tags to the templates, including one that doesn't exist and should be created
    repo.ensure_fashion_tags(std::iter::once(&fashion1.id.unwrap()), vec!["tag1", "tag2"])
        .await
        .unwrap();
    repo.ensure_fashion_tags(std::iter::once(&fashion2.id.unwrap()), vec!["tag2", "tag3"])
        .await
        .unwrap();

    // We ensure the templates have the right tags
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1", "tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We add tag1 to fashion1 again
    repo.ensure_fashion_tags(std::iter::once(&fashion1.id.unwrap()), vec!["tag1"])
        .await
        .unwrap();

    // We ensure the templates tags are unchanged
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1", "tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We remove tag1 from fashion1
    repo.remove_fashion_tags(std::iter::once(&fashion1.id.unwrap()), vec!["tag1"])
        .await
        .unwrap();

    // We ensure only fashion1 tags have changed
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We remove tag2 from fashion2
    repo.remove_fashion_tags(std::iter::once(&fashion2.id.unwrap()), vec!["tag2"])
        .await
        .unwrap();

    // We ensure only fashion2 tags have changed
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag3"]);

    // We ensure the templates themselves are unchanged
    let fetched_fashion1 = repo
        .get_fashion_by_id(&fashion1.id.unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&fetched_fashion1, fashion1);

    let fetched_fashion2 = repo
        .get_fashion_by_id(&fashion2.id.unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&fetched_fashion2, fashion2);
}

#[sqlx::test]
async fn test_rename_tag(pool: SqlitePool) {
    let repo = sqlite::Repository::new(pool);

    // We create two templates
    let fashion1 = &repo
        .insert_fashion(&Fashion::builder().name("fashion1").build())
        .await
        .unwrap();
    let fashion2 = &repo
        .insert_fashion(&Fashion::builder().name("fashion2").build())
        .await
        .unwrap();

    // We create and add tags to the templates
    repo.ensure_fashion_tags(std::iter::once(&fashion1.id.unwrap()), vec!["tag1", "tag2"])
        .await
        .unwrap();
    repo.ensure_fashion_tags(std::iter::once(&fashion2.id.unwrap()), vec!["tag2", "tag3"])
        .await
        .unwrap();

    // We ensure the templates have the right tags
    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1", "tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We rename tag1 to tag1new
    let updated_tag1 = repo.rename_tag("tag1", "tag1new").await.unwrap().unwrap();

    // We ensure the tag has been updated
    assert!(updated_tag1.updated_at.unwrap() > updated_tag1.created_at.unwrap());

    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1new", "tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We try to rename tag2 to tag3
    assert_eq!(repo.rename_tag("tag2", "tag3").await.unwrap(), None);

    // We ensure the tag hasn't been updated
    let tag2 = repo.get_tag_by_name("tag2").await.unwrap().unwrap();
    assert_eq!(tag2.updated_at.unwrap(), tag2.created_at.unwrap());

    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1new", "tag2"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2", "tag3"]);

    // We rename tag2 to tag2new
    let updated_tag2 = repo.rename_tag("tag2", "tag2new").await.unwrap().unwrap();

    // We ensure the tag has been updated
    assert!(updated_tag2.updated_at.unwrap() > updated_tag2.created_at.unwrap());

    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1new", "tag2new"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2new", "tag3"]);

    // We try to rename a tag that doesn't exist
    assert_eq!(repo.rename_tag("pikku", "peekaboo").await.unwrap(), None);

    // We ensure the tag hasn't been created or the existing tags updated
    let tags = repo
        .list_tags(StringFilters::builder().build())
        .await
        .unwrap();
    let tags: Vec<_> = tags.iter().map(|t| &t.name).collect();
    assert_eq!(tags, vec!["tag1new", "tag2new", "tag3"]);

    let tags = repo.get_fashion_tags(&fashion1.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag1new", "tag2new"]);
    let tags = repo.get_fashion_tags(&fashion2.id.unwrap()).await.unwrap();
    assert_eq!(tags, vec!["tag2new", "tag3"]);
}
