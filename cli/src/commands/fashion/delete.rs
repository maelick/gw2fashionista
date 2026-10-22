use clap::builder::ArgPredicate;
use futures::stream::FuturesOrdered;
use gw2fashionista_core::{
    app::{FashionResult, FashionService},
    domain::fashion,
};
use gw2fashionista_storage::sqlite;

use crate::{
    commands::{
        self,
        args::DataFormat,
        fashion::{args::FashionIdentifier, read_identifiers},
    },
    environment::Environment,
    partition_result,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[command(flatten)]
    id: Option<FashionIdentifier>,

    /// Input format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum,
        default_value_t = DataFormat::Auto,
        default_value_if("json", ArgPredicate::IsPresent, "json"),
        default_value_if("csv", ArgPredicate::IsPresent, "csv"))]
    format: DataFormat,

    /// Forces JSON input.
    #[arg(long, conflicts_with = "format", conflicts_with = "csv")]
    json: bool,

    /// Forces CSV input.
    #[arg(long, conflicts_with = "format", conflicts_with = "json")]
    csv: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-delete"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "fashion-delete", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let ids = self
            .read_identifiers()?
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(async |id| delete_one(&service, &id.into()).await);
        let (_, failed) = partition_result(FuturesOrdered::from_iter(ids)).await;

        anyhow::ensure!(
            failed.is_empty(),
            "Failed to delete {} templates",
            failed.len()
        );
        Ok(())
    }

    fn read_identifiers(&self) -> anyhow::Result<Vec<FashionIdentifier>> {
        Ok(match &self.id {
            Some(id) => vec![id.clone()],
            None => read_identifiers(&self.format)?.0.into(),
        })
    }
}

async fn delete_one(
    service: &FashionService<sqlite::Store>,
    id: &fashion::FashionIdentifier,
) -> FashionResult<()> {
    if let Err(err) = service.delete(id).await {
        if err.is_not_found() {
            tracing::warn!(message = "Cannot delete non existing fashion template", id = ?id);
            Ok(())
        } else {
            tracing::error!(message = "Failed to delete fashion template", id = ?id, error = ?err);
            Err(err)
        }
    } else {
        tracing::info!(message = "Fashion template successfully deleted", id = ?id);
        Ok(())
    }
}
