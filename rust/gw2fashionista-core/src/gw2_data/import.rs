use gw2lib::{Client, EndpointError, Requester, cache::InMemoryCache, rate_limit::BucketRateLimiter};
use gw2lib::model::authenticated::characters::{Character, CharacterId};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use crate::gw2_data::equipment::Equipment;

pub struct Importer<Req>
where
    Req: Requester<true, false>,
{
    req: Req,
}

impl<Req> Importer<Req>
where
    Req: Requester<true, false>,
{
    pub fn new(req: Req) -> Self {
        Importer{ req }
    }

    pub fn characters(&self) -> Result<Vec<String>, EndpointError> {
        Requester::ids::<Character, CharacterId>(&self.req)
    }

    pub fn character(&self, name: &str) -> Result<Character, EndpointError> {
        Requester::single::<Character, CharacterId>(&self.req, name.to_string())
    }

    pub fn fetch_equipment(&self, chars: &Vec<String>) -> Result<Vec<Equipment>, EndpointError> {
        let chars = if chars.is_empty() {
            &self.characters()?
        } else {
            chars
        };

        let mut result = Vec::new();
        for c in chars {
            result.extend(self.fetch_char_equipment(c.as_ref())?);
        }
        Ok(result)
    }

    pub fn fetch_char_equipment(&self, char_name: &str) -> Result<Vec<Equipment>, EndpointError> {
        let char = self.character(char_name)?;
        let tabs = char.equipment_tabs.iter().map(|t| Equipment::new(char_name, t));
        Ok(tabs.collect())
    }
}

impl Importer<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, true>> {
    pub fn with_api_key(key: &str) -> Self {
        let req = Client::default().api_key(key);
        Self::new(req)
    }
}
