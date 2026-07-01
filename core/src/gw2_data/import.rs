use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::{Client, EndpointError, Requester, cache::InMemoryCache, rate_limit::BucketRateLimiter};
use gw2lib::model::authenticated::characters::{Character, CharacterId};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use crate::gw2_data::equipment::Equipment;
use crate::gw2_data::retry::Retry;

pub struct Importer<Req>
where
    Req: Requester<true, false>,
{
    req: Req,
    retry: Retry,
}

impl<Req> Importer<Req>
where
    Req: Requester<true, false>,
{
    pub fn new(req: Req) -> Self {
        Importer{
            req,
            retry: Retry::default(),
        }
    }

    pub async fn characters(&self) -> Result<Vec<String>, EndpointError> {
        log::info!("Retrieving character list");
        self.retry.retry(async || Requester::ids::<Character, CharacterId>(&self.req).await).await
    }

    pub async fn character(&self, name: &str) -> Result<Character, EndpointError> {
        log::info!("Retrieving character data for {}", name);
        self.retry.retry(async || Requester::single::<Character, CharacterId>(&self.req, name.to_string()).await).await
    }

    pub async fn fetch_equipment(&self, chars: &Vec<String>) -> Result<Vec<Equipment>, EndpointError> {
        let chars = if chars.is_empty() {
            self.characters().await?
        } else {
            chars.clone()
        };
        let num_chars = chars.len();

        let all_tabs: Vec<_> = stream::iter(chars)
            .map(async |c| self.fetch_char_equipment(c.as_ref()).await)
            .buffered(10)
            .try_collect()
            .await?;

        let tabs: Vec<_> = all_tabs.into_iter().flatten().collect();
        log::info!("Successfully retrieved {} equipment tabs for {} characters", tabs.len(), num_chars);
        Ok(tabs)
    }

    pub async fn fetch_char_equipment(&self, char_name: &str) -> Result<Vec<Equipment>, EndpointError> {
        let char = self.character(char_name).await?;
        let tabs: Vec<_> = char.equipment_tabs.iter().map(|t| Equipment::new(char_name, t)).collect();
        log::info!("Successfully retrieved {} equipment tabs for {}", tabs.len(), char_name);
        Ok(tabs)
    }
}

impl Importer<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, true>> {
    pub fn with_api_key(key: &str) -> Self {
        let req = Client::default().api_key(key);
        Self::new(req)
    }
}
