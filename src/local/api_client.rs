use std::collections::HashMap;

use base64;
use chrono::Utc;
use eyre::Result;
use reqwest::header::AUTHORIZATION;

use crate::api::{AddHistoryRequest, CountResponse, ListHistoryResponse};
use crate::local::encryption::{decrypt, load_key};
use crate::local::history::History;
use crate::settings::Settings;

pub struct Client<'a> {
    settings: &'a Settings,
}

impl<'a> Client<'a> {
    pub const fn new(settings: &'a Settings) -> Self {
        Client { settings }
    }

    pub fn count(&self) -> Result<i64> {
        let url = format!("{}/sync/count", self.settings.local.sync_address);
        let client = reqwest::blocking::Client::new();

        let resp = client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Token {}", self.settings.local.session_token),
            )
            .send()?;

        let count = resp.json::<CountResponse>()?;

        Ok(count.count)
    }

    pub fn get_history(&self, before: chrono::DateTime<Utc>) -> Result<Vec<History>> {
        let key = load_key(self.settings)?;
        let url = format!(
            "{}/history?before={}",
            self.settings.local.sync_address,
            before.to_rfc3339()
        );
        let client = reqwest::blocking::Client::new();

        let resp = client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Token {}", self.settings.local.session_token),
            )
            .send()?;

        let history = resp.json::<ListHistoryResponse>()?;
        let history = history
            .history
            .iter()
            .map(|h| serde_json::from_str(h).expect("invalid base64"))
            .map(|h| decrypt(&h, &key).expect("failed to decrypt history! check your key"))
            .collect();

        Ok(history)
    }

    pub fn post_history(&self, history: &[AddHistoryRequest]) -> Result<()> {
        let client = reqwest::blocking::Client::new();

        let url = format!("{}/history", self.settings.local.sync_address);
        client
            .post(url)
            .json(history)
            .header(
                AUTHORIZATION,
                format!("Token {}", self.settings.local.session_token),
            )
            .send()?;

        Ok(())
    }
}
