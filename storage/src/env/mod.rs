use std::env;

use async_trait::async_trait;
use bon::Builder;
use gw2fashionista_core::ports::repositories::{SecretRepository, SecretResult};

mod error;

#[derive(Debug, Clone, Builder)]
pub struct Store {
    #[builder(default = "GW2_API_KEY".to_string())]
    api_key: String,
}

#[async_trait]
impl SecretRepository for Store {
    async fn gw2_api_key(&self) -> SecretResult<String> {
        let api_key = read_env_var(&self.api_key)?;
        if api_key.trim().is_empty() {
            Err(error::Error::Empty)?;
        }
        Ok(api_key)
    }
}

fn read_env_var(name: &str) -> error::Result<String> {
    Ok(env::var(name)?)
}
