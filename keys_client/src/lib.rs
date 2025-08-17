use std::error::Error;

use keys_lib::{ApiKey, KeyValue};
use reqwest::Client;
use serde::Serialize;
use url::Url;

#[derive(Serialize)]
struct SetValueBody<'a> {
    value: &'a str,
}

pub struct KeysClient {
    client: Client,
    server_url: Url,
    api_key: ApiKey,
}

impl KeysClient {
    pub fn new<S: AsRef<str>>(server_url: S, api_key: ApiKey) -> Self {
        let client = Client::default();
        Self {
            client,
            server_url: Url::parse(server_url.as_ref()).expect("Invalid server URL"),
            api_key,
        }
    }

    pub async fn get_value<K: AsRef<str>>(&self, key: K) -> Result<KeyValue, Box<dyn Error>> {
        let get_url = self
            .server_url
            .join(&format!("key_value/{}", key.as_ref()))?;

        Ok(self
            .client
            .get(get_url)
            .bearer_auth(self.api_key.to_base64())
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn set_value<K: AsRef<str>, V: AsRef<str>>(
        &self,
        key: K,
        value: V,
    ) -> Result<KeyValue, Box<dyn Error>> {
        let set_url = self
            .server_url
            .join(&format!("key_value/{}", key.as_ref()))?;

        Ok(self
            .client
            .post(set_url)
            .bearer_auth(self.api_key.to_base64())
            .body(
                serde_json::to_string(&SetValueBody {
                    value: value.as_ref(),
                })
                .unwrap(),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn delete_value<K: AsRef<str>>(&self, key: K) -> Result<KeyValue, Box<dyn Error>> {
        let delete_url = self
            .server_url
            .join(&format!("key_value/{}", key.as_ref()))?;

        Ok(self
            .client
            .delete(delete_url)
            .bearer_auth(self.api_key.to_base64())
            .send()
            .await?
            .json()
            .await?)
    }
}

impl Default for KeysClient {
    fn default() -> Self {
        Self::new(
            "https://api.keys.kent.software",
            ApiKey::from_base64(
                std::env::var("KEYS_API_KEY").expect("KEYS_API_KEY environment variable not set"),
            )
            .expect("Keys API key was invalid base64"),
        )
    }
}
