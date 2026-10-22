use clap::builder::ArgPredicate;
use futures::{TryStreamExt, stream::FuturesOrdered};
use gw2fashionista_core::app::FashionService;
use gw2fashionista_storage::sqlite;

use crate::{
    commands::{
        self,
        args::DataFormat,
        fashion::{args::FashionIdentifier, read_identifiers},
    },
    environment::Environment,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Tags to remove. If empty, removes all tags.
    #[arg(value_name = "TAG")]
    tags: Vec<String>,

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
        "fashion-untag"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "fashion-untag", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let ids = self.get_ids(&service).await?;
        service.untag(&ids).await?;
        Ok(())
    }

    pub async fn get_ids(
        &self,
        service: &FashionService<sqlite::Store>,
    ) -> anyhow::Result<Vec<uuid::Uuid>> {
        let ids = self
            .read_identifiers()?
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(async |id| service.resolve_id(&id.into()).await);
        Ok(FuturesOrdered::from_iter(ids).try_collect().await?)
    }

    fn read_identifiers(&self) -> anyhow::Result<Vec<FashionIdentifier>> {
        Ok(match &self.id {
            Some(id) => vec![id.clone()],
            None => read_identifiers(&self.format)?.0.into(),
        })
    }
}
