use anyhow::{Context, Result};
use hyper::client::HttpConnector;
use reqwest::Client;
use std::time::Duration;

mod get_profile;

#[derive(Clone)]
pub struct MicroserviceName {
    client: Client,
}

impl MicroserviceName {
    pub fn new() -> Result<Self> {
        Ok(MicroserviceName {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .context("build")?,
        })
    }
}
