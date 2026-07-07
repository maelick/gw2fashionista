use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::Client;
use gw2lib::model::authenticated::characters::{Character, CharacterId};
use gw2lib::model::{BulkEndpoint, EndpointWithId};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::gw2::equipment::Equipment;
use crate::gw2::error::Error;
use crate::gw2::fetch::{Fetch, Gw2LibFetcher};
use crate::gw2::retry::Retry;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Importer<T, I> {
    client: Box<dyn Fetch<T, I> + Send + Sync + 'static>,
    retry: Retry,
    buffer_size: usize,
}

impl<T, I> Importer<T, I> {
    pub fn new(client: Box<dyn Fetch<T, I> + Send + Sync + 'static>) -> Self {
        Importer {
            client,
            retry: Retry::default(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
}

impl Importer<Character, CharacterId> {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn characters(&self) -> Result<Vec<String>, Error> {
        #[cfg(feature = "tracing")]
        tracing::info!(message = "Retrieving character list");
        self.retry.start(|| self.client.ids()).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn character(&self, name: &str) -> Result<Character, Error> {
        #[cfg(feature = "tracing")]
        tracing::info!(message = "Retrieving character data");
        self.retry
            .start(|| self.client.single(name.to_string()))
            .await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub async fn fetch_equipment(&self, characters: &[String]) -> Result<Vec<Equipment>, Error> {
        let all_tabs: Vec<_> = stream::iter(characters.to_owned())
            .map(async |c| self.fetch_char_equipment(c.as_ref()).await)
            .buffered(self.buffer_size)
            .try_collect()
            .await?;

        let tabs: Vec<_> = all_tabs.into_iter().flatten().collect();

        #[cfg(feature = "tracing")]
        tracing::info!(
            message = "Successfully retrieved equipment tabs",
            num_tabs = tabs.len()
        );
        Ok(tabs)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn fetch_char_equipment(&self, char_name: &str) -> Result<Vec<Equipment>, Error> {
        let char = self.character(char_name).await?;
        let tabs: Vec<_> = char
            .equipment_tabs
            .iter()
            .map(|t| Equipment::new(char_name, t))
            .collect();

        #[cfg(feature = "tracing")]
        tracing::info!(
            message = "Successfully retrieved equipment tabs",
            num_tabs = tabs.len()
        );
        Ok(tabs)
    }
}

impl<T, I> Importer<T, I>
where
    T: EndpointWithId<IdType = I>
        + BulkEndpoint
        + Clone
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + 'static,
    I: Display + Hash + Eq + Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
{
    pub fn with_api_key(key: &str) -> Self {
        let req = Arc::new(Client::default().api_key(key));
        Self::new(Box::new(Gw2LibFetcher::new(req)))
    }
}
