use reqwest::Client;
use crate::secret::SecretStr;
use reqwest::Url;
use std::time::Duration;
use crate::error::*;

#[derive(Debug, Clone)]
pub(crate) struct PersonalAccessToken {
    pub app_id: String,
    pub secret: SecretStr,
}

#[derive(Debug, Clone)]
pub(crate) struct Api {
    client: Client,
    url: Url,
    pat: PersonalAccessToken,
}

impl Api {
    pub fn new(url: Url, pat: PersonalAccessToken) -> Result<Self> {
        let client = Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
            .https_only(true)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .build()?;
        Ok(Api {
            client,
            url,
            pat,
        })
    }
}

pub(crate) mod events;