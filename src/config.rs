use anyhow::Result;
use env::func::*;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config = Self {
            port: env::u16!("PORT", default: "8080")?,
        };

        Ok(config)
    }
}
