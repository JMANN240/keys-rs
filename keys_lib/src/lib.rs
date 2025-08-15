use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyStore {
    store: String,
}

impl KeyStore {
    pub fn new<S: Into<String>>(store: S) -> Self {
        Self {
            store: store.into(),
        }
    }

    pub fn get_store(&self) -> &str {
        &self.store
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
    store: KeyStore,
    key: String,
    value: Option<String>,
}

impl KeyValue {
    pub fn new<K: Into<String>, V: Into<String>>(
        store: KeyStore,
        key: K,
        value: Option<V>,
    ) -> Self {
        Self {
            store,
            key: key.into(),
            value: value.map(|value| value.into()),
        }
    }

    pub fn get_store(&self) -> &KeyStore {
        &self.store
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}
