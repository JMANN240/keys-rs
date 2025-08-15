use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
    key: String,
    value: Option<String>,
}

impl KeyValue {
    pub fn new<K: Into<String>, V: Into<String>>(key: K, value: Option<V>) -> Self {
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
