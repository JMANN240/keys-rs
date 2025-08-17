use std::env;

use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, Nonce},
};
use base64::{DecodeError, Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use keys_lib::{ApiKey, KeyValue};
use sqlx::{SqlitePool, query, query_as};

pub struct DbKeyValue {
    api_key_hash_base64: String,
    key: String,
    cipher_value_base64: String,
    nonce_base64: String,
}

impl From<DbKeyValue> for KeyValue {
    fn from(value: DbKeyValue) -> Self {
        let v = value.get_value();
        KeyValue::new(value.key, Some(v))
    }
}

impl DbKeyValue {
    pub fn new(
        api_key: &ApiKey,
        key: String,
        value: &str,
    ) -> Self {
        let api_key_hash_bytes = api_key.hash();
        let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);

        let master_key_base64 = env::var("MASTER_KEY").expect("MASTER_KEY environment variable is not set");
        let master_key_bytes = STANDARD_NO_PAD.decode(master_key_base64).unwrap();
        let master_key = Key::<Aes256Gcm>::from_slice(&master_key_bytes);
        let cipher = Aes256Gcm::new(master_key);
        let nonce_bytes = Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
        let cipher_value_bytes = cipher.encrypt(&nonce_bytes, value.as_bytes()).unwrap();
        let cipher_value_base64 = STANDARD_NO_PAD.encode(cipher_value_bytes);

        let nonce_base64 = STANDARD_NO_PAD.encode(nonce_bytes);

        Self { api_key_hash_base64, key, cipher_value_base64, nonce_base64 }
    }

    pub fn get_api_key_hash_base64(&self) -> &str {
        &self.api_key_hash_base64
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_cipher_value_base64(&self) -> &str {
        &self.cipher_value_base64
    }

    pub fn get_cipher_value_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD_NO_PAD.decode(self.get_cipher_value_base64())
    }

    pub fn get_nonce_base64(&self) -> &str {
        &self.nonce_base64
    }

    pub fn get_nonce_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD_NO_PAD.decode(self.get_nonce_base64())
    }

    pub fn get_value(&self) -> String {
        let master_key_base64 = env::var("MASTER_KEY").expect("MASTER_KEY environment variable is not set");
        let master_key_bytes = STANDARD_NO_PAD.decode(master_key_base64).unwrap();
        let master_key = Key::<Aes256Gcm>::from_slice(&master_key_bytes);
        let cipher = Aes256Gcm::new(master_key);
        let nonce_bytes = self.get_nonce_bytes().unwrap();
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
        let value_bytes = cipher.decrypt(nonce, self.get_cipher_value_bytes().unwrap().as_ref()).unwrap();
        String::from_utf8(value_bytes).unwrap()
    }
}

pub async fn get_db_key_values(
    pool: &SqlitePool,
    api_key: &ApiKey,
) -> Result<Vec<DbKeyValue>, sqlx::Error> {
    let api_key_hash_bytes = api_key.hash();
    let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);

    let db_key_values = query_as!(
        DbKeyValue,
        "SELECT * FROM key_values WHERE api_key_hash_base64 = ?",
        api_key_hash_base64,
    )
    .fetch_all(pool)
    .await?;

    Ok(db_key_values)
}

pub async fn get_db_key_value<K: AsRef<str>>(
    pool: &SqlitePool,
    api_key: &ApiKey,
    key: K,
) -> Result<Option<DbKeyValue>, sqlx::Error> {
    let api_key_hash_bytes = api_key.hash();
    let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);
    let key_ref = key.as_ref();

    let maybe_db_key_value = query_as!(
        DbKeyValue,
        "SELECT * FROM key_values WHERE api_key_hash_base64 = ? AND key = ?",
        api_key_hash_base64,
        key_ref,
    )
    .fetch_optional(pool)
    .await?;

    Ok(maybe_db_key_value)
}

pub async fn upsert_db_key_value(
    pool: &SqlitePool,
    db_key_value: &DbKeyValue,
) -> Result<(), sqlx::Error> {
    let api_key_hash_base64 = db_key_value.get_api_key_hash_base64();
    let key = db_key_value.get_key();
    let value = db_key_value.get_cipher_value_base64();
    let nonce = db_key_value.get_nonce_base64();

    query!(
        "INSERT INTO key_values VALUES (?, ?, ?, ?) ON CONFLICT (api_key_hash_base64, key) DO UPDATE SET cipher_value_base64 = ?, nonce_base64 = ?",
        api_key_hash_base64,
        key,
        value,
        nonce,
        value,
        nonce
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_db_key_value<K: AsRef<str>>(
    pool: &SqlitePool,
    api_key: &ApiKey,
    key: K,
) -> Result<(), sqlx::Error> {
    let api_key_hash_bytes = api_key.hash();
    let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);
    let key_ref = key.as_ref();

    query!(
        "DELETE FROM key_values WHERE api_key_hash_base64 = ? and key = ?",
        api_key_hash_base64,
        key_ref
    )
    .execute(pool)
    .await?;

    Ok(())
}
