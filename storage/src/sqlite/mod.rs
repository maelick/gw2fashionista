use async_trait::async_trait;
use gw2fashionista_core::{
    fashion::Fashion,
    ports::repositories::{self, FashionRepository},
    tag::Tag,
};
use sqlx::{
    QueryBuilder, Sqlite, SqliteConnection, SqlitePool, Transaction,
    pool::PoolConnection,
    types::{chrono, uuid},
};

use gw2fashionista_core::filters::StringFilters;

mod error;
mod models;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn acquire_conn(&self) -> error::Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }

    async fn begin_transaction(&self) -> error::Result<Transaction<'_, Sqlite>> {
        Ok(self.pool.begin().await?)
    }
}

#[async_trait]
impl FashionRepository for Repository {
    async fn insert_fashion(&self, fashion: &Fashion) -> repositories::Result<Fashion> {
        let mut conn = self.acquire_conn().await?;
        Ok(insert_fashion(&mut conn, fashion).await?)
    }

    async fn update_fashion(&self, fashion: &Fashion) -> repositories::Result<Fashion> {
        let mut conn = self.acquire_conn().await?;
        Ok(update_fashion(&mut conn, fashion).await?)
    }

    async fn get_fashion_by_id(&self, id: &uuid::Uuid) -> repositories::Result<Fashion> {
        let mut conn = self.acquire_conn().await?;
        Ok(get_fashion_by_id(&mut conn, id).await?)
    }

    async fn get_fashion_by_name(
        &self,
        name: &str,
        character: Option<&str>,
    ) -> repositories::Result<Fashion> {
        let mut conn = self.acquire_conn().await?;
        Ok(get_fashion_by_name(&mut conn, name, character).await?)
    }

    async fn list_fashions(&self) -> repositories::Result<Vec<Fashion>> {
        let mut conn = self.acquire_conn().await?;
        Ok(list_fashions(&mut conn).await?)
    }

    async fn upsert_tag(&self, name: &str) -> repositories::Result<Option<Tag>> {
        let mut conn = self.acquire_conn().await?;
        Ok(upsert_tag(&mut conn, name).await?)
    }

    async fn ensure_tag(&self, name: &str) -> repositories::Result<Tag> {
        let mut tx = self.begin_transaction().await?;
        let tag = ensure_tag(&mut tx, name).await?;
        commit(tx).await?;
        Ok(tag)
    }

    async fn rename_tag(&self, from: &str, to: &str) -> repositories::Result<Tag> {
        let mut conn = self.acquire_conn().await?;
        Ok(rename_tag(&mut conn, from, to).await?)
    }

    async fn get_tag_by_id(&self, id: &uuid::Uuid) -> repositories::Result<Tag> {
        let mut conn = self.acquire_conn().await?;
        Ok(get_tag_by_id(&mut conn, id).await?)
    }

    async fn get_tag_by_name(&self, name: &str) -> repositories::Result<Tag> {
        let mut conn = self.acquire_conn().await?;
        Ok(get_tag_by_name(&mut conn, name).await?)
    }

    async fn list_tags(&self, filters: StringFilters) -> repositories::Result<Vec<Tag>> {
        let mut conn = self.acquire_conn().await?;
        Ok(list_tags(&mut conn, SqliteStringFilters(filters)).await?)
    }

    async fn replace_tags(
        &self,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
        with: &str,
    ) -> repositories::Result<()> {
        let mut tx = self.begin_transaction().await?;
        let with_id = resolve_tag_id(&mut tx, with).await?;
        for tag in tags.into_iter().map(Into::into) {
            let tag_id = resolve_tag_id(&mut tx, &tag).await?;
            replace_tag(&mut tx, &tag_id, &with_id).await?;
        }
        commit(tx).await?;
        Ok(())
    }

    async fn clean_tags(&self) -> repositories::Result<()> {
        let mut conn = self.acquire_conn().await?;
        Ok(clean_tags(&mut conn).await?)
    }

    async fn get_fashion_tags(&self, fashion_id: &uuid::Uuid) -> repositories::Result<Vec<String>> {
        let mut conn = self.acquire_conn().await?;
        Ok(get_fashion_tags(&mut conn, fashion_id).await?)
    }

    async fn ensure_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> repositories::Result<()> {
        let fashion_ids: Vec<_> = fashion_ids.into_iter().collect();
        let mut tx = self.begin_transaction().await?;
        for tag in tags.into_iter().map(Into::into) {
            upsert_tag(&mut tx, &tag).await?;
            for fashion_id in &fashion_ids {
                add_fashion_tag(&mut tx, fashion_id, &tag).await?;
            }
        }
        commit(tx).await?;
        Ok(())
    }

    async fn remove_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> repositories::Result<()> {
        let fashion_ids: Vec<_> = fashion_ids.into_iter().collect();
        let mut tx = self.begin_transaction().await?;
        for tag in tags.into_iter().map(Into::into) {
            for fashion_id in &fashion_ids {
                remove_fashion_tag(&mut tx, fashion_id, &tag).await?;
            }
        }
        commit(tx).await?;
        Ok(())
    }
}

struct SqliteStringFilters(StringFilters);

impl SqliteStringFilters {
    pub fn patterns(&self) -> impl Iterator<Item = String> {
        self.0
            .substrings
            .iter()
            .map(|s| format!("%{}%", s))
            .chain(self.0.prefix.as_ref().map(|s| format!("{}%", s)))
            .chain(self.0.suffix.as_ref().map(|s| format!("%{}", s)))
    }
}

async fn insert_fashion(conn: &mut SqliteConnection, fashion: &Fashion) -> error::Result<Fashion> {
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

async fn update_fashion(conn: &mut SqliteConnection, fashion: &Fashion) -> error::Result<Fashion> {
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

async fn get_fashion_by_id(conn: &mut SqliteConnection, id: &uuid::Uuid) -> error::Result<Fashion> {
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
) -> error::Result<Fashion> {
    sqlx::query_as::<'_, _, models::Fashion>(
        r#"SELECT * FROM fashion WHERE name = ? AND character = ?"#,
    )
    .bind(name)
    .bind(character.unwrap_or_default())
    .fetch_one(conn)
    .await?
    .try_into()
}

async fn list_fashions(conn: &mut SqliteConnection) -> error::Result<Vec<Fashion>> {
    sqlx::query_as::<'_, _, models::Fashion>("SELECT * FROM fashion")
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(Fashion::try_from)
        .collect()
}

async fn upsert_tag(conn: &mut SqliteConnection, name: &str) -> error::Result<Option<Tag>> {
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

async fn ensure_tag(conn: &mut SqliteConnection, name: &str) -> error::Result<Tag> {
    if let Some(created_tag) = upsert_tag(&mut *conn, name).await? {
        Ok(created_tag)
    } else {
        Ok(get_tag_by_name(&mut *conn, name).await?)
    }
}

async fn rename_tag(conn: &mut SqliteConnection, from: &str, to: &str) -> error::Result<Tag> {
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

async fn get_tag_by_id(conn: &mut SqliteConnection, id: &uuid::Uuid) -> error::Result<Tag> {
    Ok(
        sqlx::query_as::<'_, _, models::Tag>(r#"SELECT * FROM tag WHERE id = ?"#)
            .bind(id.hyphenated())
            .fetch_one(conn)
            .await?
            .into(),
    )
}

async fn get_tag_by_name(conn: &mut SqliteConnection, name: &str) -> error::Result<Tag> {
    Ok(
        sqlx::query_as::<'_, _, models::Tag>(r#"SELECT * FROM tag WHERE name = ?"#)
            .bind(name)
            .fetch_one(conn)
            .await?
            .into(),
    )
}

async fn resolve_tag_id(conn: &mut SqliteConnection, name: &str) -> error::Result<uuid::Uuid> {
    let res = sqlx::query!(r#"SELECT id FROM tag WHERE name = ?"#, name)
        .fetch_one(conn)
        .await?;
    Ok(res.id.try_into()?)
}

async fn list_tags(
    conn: &mut SqliteConnection,
    filters: SqliteStringFilters,
) -> error::Result<Vec<Tag>> {
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
) -> error::Result<()> {
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

async fn clean_tags(conn: &mut SqliteConnection) -> error::Result<()> {
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
) -> error::Result<Vec<String>> {
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
) -> error::Result<()> {
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
) -> error::Result<()> {
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

async fn commit(tx: Transaction<'_, Sqlite>) -> error::Result<()> {
    Ok(tx.commit().await?)
}
