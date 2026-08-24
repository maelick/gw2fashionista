use gw2fashionista_core::domain::fashion::Fashion;
use sqlx::{Acquire, Sqlite, SqlitePool, types::uuid};

use crate::models;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_fashion(&self, fashion: &Fashion) -> crate::Result<Fashion> {
        insert_fashion(&self.pool, fashion).await
    }

    pub async fn get_fashion_by_id(&self, id: uuid::Uuid) -> crate::Result<Option<Fashion>> {
        get_fashion_by_id(&self.pool, id).await
    }

    pub async fn get_fashion_by_name(
        &self,
        name: &str,
        character: Option<&str>,
    ) -> crate::Result<Option<Fashion>> {
        get_fashion_by_name(&self.pool, name, character).await
    }

    pub async fn list_fashions(&self) -> crate::Result<Vec<Fashion>> {
        list_fashions(&self.pool).await
    }

    pub async fn upsert_tag(&self, name: &str) -> crate::Result<Option<models::Tag>> {
        upsert_tag(&self.pool, name).await
    }

    pub async fn ensure_tag(&self, name: &str) -> crate::Result<models::Tag> {
        let mut tx = self.pool.begin().await?;
        let tag = ensure_tag(&mut *tx, name).await?;
        tx.commit().await?;
        Ok(tag)
    }

    pub async fn get_tag_by_id(&self, id: &uuid::Uuid) -> crate::Result<Option<models::Tag>> {
        get_tag_by_id(&self.pool, id).await
    }

    pub async fn get_tag_by_name(&self, name: &str) -> crate::Result<Option<models::Tag>> {
        get_tag_by_name(&self.pool, name).await
    }
}

async fn insert_fashion<'a, A>(conn: A, fashion: &Fashion) -> crate::Result<Fashion>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let model: models::Fashion = fashion.into();
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as!(
        models::Fashion,
        r#"INSERT INTO fashion (
                id,
                name,
                description,
                character,
                wardrobe_template,
                travel_template
            ) VALUES (?, ?, ?, ?, ?, ?)
            RETURNING 
                id as "id: _",
                name,
                description,
                character,
                wardrobe_template,
                travel_template,
                created_at as "created_at: _",
                updated_at as "updated_at: _""#,
        model.id,
        model.name,
        model.description,
        model.character,
        model.wardrobe_template,
        model.travel_template,
    )
    .fetch_one(&mut *conn)
    .await?
    .try_into()?)
}

async fn get_fashion_by_id<'a, A>(conn: A, id: uuid::Uuid) -> crate::Result<Option<Fashion>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    sqlx::query_as!(
        models::Fashion,
        r#"SELECT
                id as "id: _",
                name,
                description,
                character,
                wardrobe_template,
                travel_template,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM fashion WHERE id = ?"#,
        id.hyphenated()
    )
    .fetch_optional(&mut *conn)
    .await?
    .map(Fashion::try_from)
    .transpose()
}

async fn get_fashion_by_name<'a, A>(
    conn: A,
    name: &str,
    character: Option<&str>,
) -> crate::Result<Option<Fashion>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    sqlx::query_as!(
        models::Fashion,
        r#"SELECT
                id as "id: _",
                name,
                description,
                character,
                wardrobe_template,
                travel_template,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM fashion WHERE name = ? AND character = ?"#,
        name,
        &character.unwrap_or_default(),
    )
    .fetch_optional(&mut *conn)
    .await?
    .map(Fashion::try_from)
    .transpose()
}

async fn list_fashions<'a, A>(conn: A) -> crate::Result<Vec<Fashion>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    sqlx::query_as::<'_, _, models::Fashion>("SELECT * FROM fashion")
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(Fashion::try_from)
        .collect()
}

async fn upsert_tag<'a, A>(conn: A, name: &str) -> crate::Result<Option<models::Tag>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let id = uuid::Uuid::now_v7();
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as!(
        models::Tag,
        r#"INSERT INTO tag (
                id,
                name
            ) VALUES (?, ?)
            ON CONFLICT(name)
            DO NOTHING
            RETURNING
                id as "id: _",
                name,
                created_at as "created_at: _",
                updated_at as "updated_at: _""#,
        id.hyphenated(),
        name,
    )
    .fetch_optional(&mut *conn)
    .await?)
}

async fn ensure_tag<'a, A>(conn: A, name: &str) -> crate::Result<models::Tag>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    if let Some(created_tag) = upsert_tag(&mut *conn, name).await? {
        Ok(created_tag)
    } else {
        get_tag_by_name(&mut *conn, name)
            .await?
            .ok_or(crate::Error::NotFound) // should never happen
    }
}

async fn get_tag_by_id<'a, A>(conn: A, id: &uuid::Uuid) -> crate::Result<Option<models::Tag>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as!(
        models::Tag,
        r#"SELECT
                id as "id: _",
                name,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM tag WHERE id = ?"#,
        id.hyphenated()
    )
    .fetch_optional(&mut *conn)
    .await?)
}

async fn get_tag_by_name<'a, A>(conn: A, name: &str) -> crate::Result<Option<models::Tag>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as!(
        models::Tag,
        r#"SELECT
                id as "id: _",
                name,
                created_at as "created_at: _",
                updated_at as "updated_at: _"
            FROM tag WHERE name = ?"#,
        name
    )
    .fetch_optional(&mut *conn)
    .await?)
}
