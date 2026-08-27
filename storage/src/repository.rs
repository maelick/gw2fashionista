use gw2fashionista_core::domain::fashion::Fashion;
use sqlx::{Acquire, QueryBuilder, Sqlite, SqlitePool, types::uuid};

use crate::models;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct StringFilters {
    prefix: Option<String>,
    suffix: Option<String>,
    substrings: Vec<String>,
}

impl StringFilters {
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn with_substrings(
        mut self,
        substrings: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.substrings = substrings.into_iter().map(Into::into).collect();
        self
    }

    pub fn patterns(&self) -> impl Iterator<Item = String> {
        self.substrings
            .iter()
            .map(|s| format!("%{}%", s))
            .chain(self.prefix.as_ref().map(|s| format!("{}%", s)).into_iter())
            .chain(self.suffix.as_ref().map(|s| format!("%{}", s)).into_iter())
    }
}

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

    pub async fn list_tags(&self, filters: StringFilters) -> crate::Result<Vec<models::Tag>> {
        list_tags(&self.pool, filters).await
    }
}

async fn insert_fashion<'a, A>(conn: A, fashion: &Fashion) -> crate::Result<Fashion>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let model: models::Fashion = fashion.into();
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as::<'_, _, models::Fashion>(
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
    .fetch_one(&mut *conn)
    .await?
    .try_into()?)
}

async fn get_fashion_by_id<'a, A>(conn: A, id: uuid::Uuid) -> crate::Result<Option<Fashion>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    sqlx::query_as::<'_, _, models::Fashion>(r#"SELECT * FROM fashion WHERE id = ?"#)
        .bind(id.hyphenated())
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
    sqlx::query_as::<'_, _, models::Fashion>(
        r#"SELECT * FROM fashion WHERE name = ? AND character = ?"#,
    )
    .bind(name)
    .bind(&character.unwrap_or_default())
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
    Ok(sqlx::query_as(
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
    Ok(sqlx::query_as(r#"SELECT * FROM tag WHERE id = ?"#)
        .bind(id.hyphenated())
        .fetch_optional(&mut *conn)
        .await?)
}

async fn get_tag_by_name<'a, A>(conn: A, name: &str) -> crate::Result<Option<models::Tag>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    Ok(sqlx::query_as(r#"SELECT * FROM tag WHERE name = ?"#)
        .bind(name)
        .fetch_optional(&mut *conn)
        .await?)
}

async fn list_tags<'a, A>(conn: A, filters: StringFilters) -> crate::Result<Vec<models::Tag>>
where
    A: Acquire<'a, Database = Sqlite>,
{
    let mut conn = conn.acquire().await?;
    Ok(list_tags_query(filters.patterns())
        .build_query_as()
        .fetch_all(&mut *conn)
        .await?)
}

fn list_tags_query(patterns: impl Iterator<Item = String>) -> QueryBuilder<Sqlite> {
    let mut query = QueryBuilder::new(r#"SELECT * FROM tag WHERE 1 = 1"#);
    for p in patterns {
        query.push(" AND name LIKE ");
        query.push_bind(p);
    }
    query
}
