use base64::{engine::general_purpose::URL_SAFE, DecodeSliceError, Engine as _};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
    key: String,
    value: Option<String>,
}

impl KeyValue {
    pub fn new<K: Into<String>, V: Into<String>>(
        key: K,
        value: Option<V>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.map(|value| value.into()),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

pub struct ApiKey {
    bytes: [u8; 32],
}

impl ApiKey {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    pub fn from_base64(base64_api_key: &str) -> Result<Self, DecodeSliceError> {
        let mut bytes = [0u8; 32];
        URL_SAFE.decode_slice(base64_api_key, &mut bytes)?;
        Ok(Self::from_bytes(bytes))
    }

    pub fn to_base64(&self) -> String {
        URL_SAFE.encode(self.bytes)
    }

    pub fn hash(&self) -> Vec<u8> {
        hex::decode(sha256::digest(&self.bytes)).expect("SHA256 digest contained invalid hex")
    }
}
