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

    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: Option<FashionIdentifier>,

    /// Input format. Auto is based on whether stdin is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-untag"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-untag", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let ids = self.get_ids(&service).await?;
        service.untag(&ids).await?;
        Ok(())
    }

    pub async fn get_ids(
        &self,
        service: &FashionService<sqlite::Repository>,
    ) -> anyhow::Result<Vec<uuid::Uuid>> {
        let ids = self
            .read_identifiers()
            .await?
            .into_iter()
            .filter(|id| !id.is_empty())
            .map(async |id| service.resolve_id(&id.into()).await);
        Ok(FuturesOrdered::from_iter(ids).try_collect().await?)
    }

    async fn read_identifiers(&self) -> anyhow::Result<Vec<FashionIdentifier>> {
        Ok(match &self.id {
            Some(id) => vec![id.clone()],
            None => read_identifiers(&self.format)?.0.into(),
        })
    }
}
