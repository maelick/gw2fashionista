use async_trait::async_trait;
use gw2fashionista_core::domain::{fashion::Fashion, tag::Tag};
use sqlx::{
    QueryBuilder, Sqlite, SqliteConnection, SqlitePool,
    types::{chrono, uuid},
};

use crate::StringFilters;

mod models;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl crate::Repository for Repository {
    async fn insert_fashion(&self, fashion: &Fashion) -> crate::Result<Fashion> {
        let mut conn = self.pool.acquire().await?;
        insert_fashion(&mut conn, fashion).await
    }

    async fn update_fashion(&self, fashion: &Fashion) -> crate::Result<Fashion> {
        let mut conn = self.pool.acquire().await?;
        update_fashion(&mut conn, fashion).await
    }

    async fn get_fashion_by_id(&self, id: &uuid::Uuid) -> crate::Result<Fashion> {
        let mut conn = self.pool.acquire().await?;
        get_fashion_by_id(&mut conn, id).await
    }

    async fn get_fashion_by_name(
        &self,
        name: &str,
        character: Option<&str>,
    ) -> crate::Result<Fashion> {
        let mut conn = self.pool.acquire().await?;
        get_fashion_by_name(&mut conn, name, character).await
    }

    async fn list_fashions(&self) -> crate::Result<Vec<Fashion>> {
        let mut conn = self.pool.acquire().await?;
        list_fashions(&mut conn).await
    }

    async fn upsert_tag(&self, name: &str) -> crate::Result<Option<Tag>> {
        let mut conn = self.pool.acquire().await?;
        upsert_tag(&mut conn, name).await
    }

    async fn ensure_tag(&self, name: &str) -> crate::Result<Tag> {
        let mut tx = self.pool.begin().await?;
        let tag = ensure_tag(&mut tx, name).await?;
        tx.commit().await?;
        Ok(tag)
    }

    async fn rename_tag(&self, from: &str, to: &str) -> crate::Result<Tag> {
        let mut conn = self.pool.acquire().await?;
        rename_tag(&mut conn, from, to).await
    }

    async fn get_tag_by_id(&self, id: &uuid::Uuid) -> crate::Result<Tag> {
        let mut conn = self.pool.acquire().await?;
        get_tag_by_id(&mut conn, id).await
    }

    async fn get_tag_by_name(&self, name: &str) -> crate::Result<Tag> {
        let mut conn = self.pool.acquire().await?;
        get_tag_by_name(&mut conn, name).await
    }

    async fn list_tags(&self, filters: StringFilters) -> crate::Result<Vec<Tag>> {
        let mut conn = self.pool.acquire().await?;
        list_tags(&mut conn, filters).await
    }

    async fn replace_tags(
        &self,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
        with: &str,
    ) -> crate::Result<()> {
        let mut tx = self.pool.begin().await?;
        let with_id = resolve_tag_id(&mut tx, with).await?;
        for tag in tags.into_iter().map(Into::into) {
            let tag_id = resolve_tag_id(&mut tx, &tag).await?;
            replace_tag(&mut tx, &tag_id, &with_id).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn clean_tags(&self) -> crate::Result<()> {
        let mut conn = self.pool.acquire().await?;
        clean_tags(&mut conn).await
    }

    async fn get_fashion_tags(&self, fashion_id: &uuid::Uuid) -> crate::Result<Vec<String>> {
        let mut conn = self.pool.acquire().await?;
        get_fashion_tags(&mut conn, fashion_id).await
    }

    async fn ensure_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> crate::Result<()> {
        let fashion_ids: Vec<_> = fashion_ids.into_iter().collect();
        let mut tx = self.pool.begin().await?;
        for tag in tags.into_iter().map(Into::into) {
            upsert_tag(&mut tx, &tag).await?;
            for fashion_id in &fashion_ids {
                add_fashion_tag(&mut tx, fashion_id, &tag).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    async fn remove_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> crate::Result<()> {
        let fashion_ids: Vec<_> = fashion_ids.into_iter().collect();
        let mut tx = self.pool.begin().await?;
        for tag in tags.into_iter().map(Into::into) {
            for fashion_id in &fashion_ids {
                remove_fashion_tag(&mut tx, fashion_id, &tag).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
}

async fn insert_fashion(conn: &mut SqliteConnection, fashion: &Fashion) -> crate::Result<Fashion> {
    let model: models::Fashion = fashion.into();
    sqlx::query_as::<'_, _, models::Fashion>(
        r#"INSERT INTO fashion (
                id,
                name,
                description,
                character,
                wardrobe_template,
                travel_template
            ) VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *"#,
    )
    .bind(model.id)
    .bind(model.name)
    .bind(model.description)
    .bind(model.character)
    .bind(model.wardrobe_template)
    .bind(model.travel_template)
    .fetch_one(conn)
    .await?
    .try_into()
}

async fn update_fashion(conn: &mut SqliteConnection, fashion: &Fashion) -> crate::Result<Fashion> {
    let model: models::Fashion = fashion.into();
    sqlx::query_as::<'_, _, models::Fashion>(
            r#"UPDATE OR ABORT fashion
            SET name = ?, description = ?, character = ?, wardrobe_template = ?, travel_template = ?, updated_at = ?
            WHERE id = ?
            RETURNING *"#,
        )
        .bind(model.name)
        .bind(model.description)
        .bind(model.character)
        .bind(model.wardrobe_template)
        .bind(model.travel_template)
        .bind(chrono::Utc::now())
        .bind(model.id)
        .fetch_one(conn)
        .await?
        .try_into()
}

async fn get_fashion_by_id(conn: &mut SqliteConnection, id: &uuid::Uuid) -> crate::Result<Fashion> {
    sqlx::query_as::<'_, _, models::Fashion>(r#"SELECT * FROM fashion WHERE id = ?"#)
        .bind(id.hyphenated())
        .fetch_one(conn)
        .await?
        .try_into()
}

async fn get_fashion_by_name(
    conn: &mut SqliteConnection,
    name: &str,
    character: Option<&str>,
) -> crate::Result<Fashion> {
    sqlx::query_as::<'_, _, models::Fashion>(
        r#"SELECT * FROM fashion WHERE name = ? AND character = ?"#,
    )
    .bind(name)
    .bind(character.unwrap_or_default())
    .fetch_one(conn)
    .await?
    .try_into()
}

async fn list_fashions(conn: &mut SqliteConnection) -> crate::Result<Vec<Fashion>> {
    sqlx::query_as::<'_, _, models::Fashion>("SELECT * FROM fashion")
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(Fashion::try_from)
        .collect()
}

async fn upsert_tag(conn: &mut SqliteConnection, name: &str) -> crate::Result<Option<Tag>> {
    let id = uuid::Uuid::now_v7();
    Ok(sqlx::query_as::<'_, _, models::Tag>(
        r#"INSERT INTO tag (
                id,
                name
            ) VALUES (?, ?)
            ON CONFLICT(name)
            DO NOTHING
            RETURNING *"#,
    )
    .bind(id.hyphenated())
    .bind(name)
    .fetch_optional(conn)
    .await?
    .map(Tag::from))
}

async fn ensure_tag(conn: &mut SqliteConnection, name: &str) -> crate::Result<Tag> {
    if let Some(created_tag) = upsert_tag(&mut *conn, name).await? {
        Ok(created_tag)
    } else {
        Ok(get_tag_by_name(&mut *conn, name).await?)
    }
}

async fn rename_tag(conn: &mut SqliteConnection, from: &str, to: &str) -> crate::Result<Tag> {
    Ok(sqlx::query_as::<'_, _, models::Tag>(
        r#"UPDATE OR ABORT tag
        SET name = ?, updated_at = ?
        WHERE name = ?
        RETURNING *"#,
    )
    .bind(to)
    .bind(chrono::Utc::now())
    .bind(from)
    .fetch_one(conn)
    .await?
    .into())
}

async fn get_tag_by_id(conn: &mut SqliteConnection, id: &uuid::Uuid) -> crate::Result<Tag> {
    Ok(
        sqlx::query_as::<'_, _, models::Tag>(r#"SELECT * FROM tag WHERE id = ?"#)
            .bind(id.hyphenated())
            .fetch_one(conn)
            .await?
            .into(),
    )
}

async fn get_tag_by_name(conn: &mut SqliteConnection, name: &str) -> crate::Result<Tag> {
    Ok(
        sqlx::query_as::<'_, _, models::Tag>(r#"SELECT * FROM tag WHERE name = ?"#)
            .bind(name)
            .fetch_one(conn)
            .await?
            .into(),
    )
}

async fn resolve_tag_id(conn: &mut SqliteConnection, name: &str) -> crate::Result<uuid::Uuid> {
    let res = sqlx::query!(r#"SELECT id FROM tag WHERE name = ?"#, name)
        .fetch_one(conn)
        .await?;
    Ok(res.id.try_into()?)
}

async fn list_tags(conn: &mut SqliteConnection, filters: StringFilters) -> crate::Result<Vec<Tag>> {
    Ok(list_tags_query(filters.patterns())
        .build_query_as::<'_, models::Tag>()
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(Tag::from)
        .collect())
}

async fn replace_tag(
    conn: &mut SqliteConnection,
    tag: &uuid::Uuid,
    with: &uuid::Uuid,
) -> crate::Result<()> {
    sqlx::query!(
        r#"UPDATE OR REPLACE fashion_tag
        SET tag_id = ?
        WHERE tag_id = ?"#,
        with.hyphenated(),
        tag.hyphenated(),
    )
    .execute(conn)
    .await?;
    Ok(())
}

async fn clean_tags(conn: &mut SqliteConnection) -> crate::Result<()> {
    sqlx::query!(
        r#"DELETE FROM tag
        WHERE NOT EXISTS (
            SELECT 1 FROM fashion_tag WHERE fashion_tag.tag_id = tag.id
        )"#
    )
    .execute(conn)
    .await?;
    Ok(())
}

fn list_tags_query(patterns: impl Iterator<Item = String>) -> QueryBuilder<Sqlite> {
    let mut query = QueryBuilder::new(r#"SELECT * FROM tag WHERE 1 = 1"#);
    for p in patterns {
        query.push(" AND name LIKE ");
        query.push_bind(p);
    }
    query
}

async fn get_fashion_tags(
    conn: &mut SqliteConnection,
    fashion_id: &uuid::Uuid,
) -> crate::Result<Vec<String>> {
    let query = sqlx::query!(
        r#"SELECT name FROM tag
        JOIN fashion_tag ON tag.id = fashion_tag.tag_id
        WHERE fashion_tag.fashion_id = ?"#,
        fashion_id.hyphenated()
    );
    Ok(query.map(|r| r.name).fetch_all(conn).await?)
}

async fn add_fashion_tag(
    conn: &mut SqliteConnection,
    fashion_id: &uuid::Uuid,
    tag: impl Into<String>,
) -> crate::Result<()> {
    let query = sqlx::query!(
        r#"INSERT INTO fashion_tag (
                fashion_id,
                tag_id
            )
            SELECT ?, id
            FROM tag WHERE name = ?
            ON CONFLICT(fashion_id, tag_id)
            DO NOTHING"#,
        fashion_id.hyphenated(),
        tag.into(),
    );
    query.execute(conn).await?;
    Ok(())
}

async fn remove_fashion_tag(
    conn: &mut SqliteConnection,
    fashion_id: &uuid::Uuid,
    tag: impl Into<String>,
) -> crate::Result<()> {
    let query = sqlx::query!(
        r#"DELETE FROM fashion_tag
            WHERE fashion_id = ?
            AND tag_id IN (
                SELECT id FROM tag WHERE name = ?
            )"#,
        fashion_id.hyphenated(),
        tag.into(),
    );
    query.execute(conn).await?;
    Ok(())
}
