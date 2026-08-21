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
}

pub async fn insert_fashion<'a, A>(conn: A, fashion: &Fashion) -> crate::Result<Fashion>
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

pub async fn get_fashion_by_id<'a, A>(conn: A, id: uuid::Uuid) -> crate::Result<Option<Fashion>>
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
    .map(|r| r.try_into())
    .transpose()
}

pub async fn get_fashion_by_name<'a, A>(
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
    .map(|r| r.try_into())
    .transpose()
}
