use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::{Client, EndpointError, Requester, cache::InMemoryCache, rate_limit::BucketRateLimiter};
use gw2lib::model::authenticated::characters::{Character, CharacterId};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use crate::gw2_data::equipment::Equipment;
use crate::gw2_data::retry::Retry;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Importer<Req>
where
    Req: Requester<true, false>,
{
    req: Req,
    retry: Retry,
    buffer_size: usize,
}

impl<Req> Importer<Req>
where
    Req: Requester<true, false>,
{
    pub fn new(req: Req) -> Self {
        Importer{
            req,
            retry: Retry::default(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        return self
    }

    pub async fn characters(&self) -> Result<Vec<String>, EndpointError> {
        #[cfg(feature = "tracing")]
        tracing::info!("Retrieving character list");
        self.retry.start(|| Requester::ids::<Character, CharacterId>(&self.req)).await
    }

    pub async fn character(&self, name: &str) -> Result<Character, EndpointError> {
        #[cfg(feature = "tracing")]
        tracing::info!("Retrieving character data for {}", name);
        self.retry.start(|| Requester::single::<Character, CharacterId>(&self.req, name.to_string())).await
    }

    pub async fn fetch_equipment(&self, characters: &Vec<String>) -> Result<Vec<Equipment>, EndpointError> {
        let all_tabs: Vec<_> = stream::iter(characters.clone())
            .map(async |c| self.fetch_char_equipment(c.as_ref()).await)
            .buffered(self.buffer_size)
            .try_collect()
            .await?;

        let tabs: Vec<_> = all_tabs.into_iter().flatten().collect();
        #[cfg(feature = "tracing")]
        tracing::info!("Successfully retrieved {} equipment tabs for {} characters", tabs.len(), characters.len());
        Ok(tabs)
    }

    pub async fn fetch_char_equipment(&self, char_name: &str) -> Result<Vec<Equipment>, EndpointError> {
        let char = self.character(char_name).await?;
        let tabs: Vec<_> = char.equipment_tabs.iter().map(|t| Equipment::new(char_name, t)).collect();
        #[cfg(feature = "tracing")]
        tracing::info!("Successfully retrieved {} equipment tabs for {}", tabs.len(), char_name);
        Ok(tabs)
    }
}

impl Importer<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, true>> {
    pub fn with_api_key(key: &str) -> Self {
        let req = Client::default().api_key(key);
        Self::new(req)
    }
}
