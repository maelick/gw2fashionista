use std::{path::PathBuf, sync::Arc};

use bon::Builder;
use gw2fashionista_core::{app::FashionService, config};
use gw2fashionista_storage::sqlite;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Builder)]
pub struct Environment {
    #[builder(into)]
    db_path: Option<PathBuf>,

    sql_pool: Option<SqlitePool>,

    fashion_repo: Option<Arc<sqlite::Repository>>,

    #[builder(default)]
    dirs: config::Config,
}

impl Environment {
    pub fn db_path(&self) -> &PathBuf {
        self.db_path.as_ref().unwrap_or(&self.dirs.db_path)
    }

    async fn connect_sqlite(&self) -> sqlx::Result<SqlitePool> {
        let db_path = self.db_path().to_str();
        tracing::debug!(message = "Opening SQLite database", path = db_path);
        sqlite::init(db_path.unwrap()).await
    }

    async fn sql_pool(&mut self) -> sqlx::Result<SqlitePool> {
        if self.sql_pool.is_none() {
            let pool = self.connect_sqlite().await?;
            self.sql_pool = Some(pool.clone());
            Ok(pool)
        } else {
            Ok(self.sql_pool.clone().unwrap())
        }
    }

    async fn fashion_repo(&mut self) -> anyhow::Result<Arc<sqlite::Repository>> {
        if self.fashion_repo.is_none() {
            let pool = self.sql_pool().await?;
            self.fashion_repo = Some(Arc::new(sqlite::Repository::new(pool)));
        }
        Ok(self.fashion_repo.clone().unwrap())
    }

    pub async fn fashion_service(&mut self) -> anyhow::Result<FashionService<sqlite::Repository>> {
        let repo = self.fashion_repo().await?;
        Ok(FashionService::new(repo))
    }
}
