use gw2fashionista_core::domain::fashion::Fashion;
use sqlx::{SqlitePool, types::uuid};

use crate::models;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert_fashion(&self, fashion: &Fashion) -> crate::Result<Fashion> {
        let model: models::Fashion = fashion.into();
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
        .fetch_one(&self.pool)
        .await?
        .try_into()?)
    }

    pub async fn get_fashion_by_id(&self, id: uuid::Uuid) -> crate::Result<Option<Fashion>> {
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
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.try_into())
        .transpose()
    }

    pub async fn get_fashion_by_name(
        &self,
        name: &str,
        character: Option<&str>,
    ) -> crate::Result<Option<Fashion>> {
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
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.try_into())
        .transpose()
    }
}
