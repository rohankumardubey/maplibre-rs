//! Module which is used target platform is not web related.

use async_trait::async_trait;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_middleware_cache::managers::CACacheManager;
use reqwest_middleware_cache::{Cache, CacheMode};

use crate::error::Error;
use crate::io::HttpFetcher;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err.to_string())
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        Error::Network(err.to_string())
    }
}

pub struct PlatformHttpFetcher {
    client: ClientWithMiddleware,
}

#[async_trait]
impl HttpFetcher for PlatformHttpFetcher {
    fn new() -> Self {
        let client = ClientBuilder::new(Client::new())
            .with(Cache {
                mode: CacheMode::Default,
                cache_manager: CACacheManager::default(),
            })
            .build();
        Self { client }
    }

    async fn fetch(&self, url: &str) -> Result<Vec<u8>, Error> {
        let body = self.client.get(url).send().await?.bytes().await?;
        Ok(Vec::from(body.as_ref()))
    }
}
