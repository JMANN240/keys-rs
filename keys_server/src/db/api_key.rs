use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use keys_lib::ApiKey;
use sqlx::{SqlitePool, query};

pub struct DbApiKey {
    api_key_hash_base64: String,
}

impl DbApiKey {
    pub fn new(
        api_key: &ApiKey,
    ) -> Self {
        let api_key_hash_bytes = api_key.hash();
        let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);

        Self { api_key_hash_base64 }
    }

    pub fn get_api_key_hash_base64(&self) -> &str {
        &self.api_key_hash_base64
    }
}

pub async fn insert_db_api_key(
    pool: &SqlitePool,
    db_api_key: &DbApiKey,
) -> Result<(), sqlx::Error> {
    let api_key_hash_base64 = db_api_key.get_api_key_hash_base64();

    query!(
        "INSERT INTO api_keys VALUES (?)",
        api_key_hash_base64,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_db_api_key(
    pool: &SqlitePool,
    api_key: &ApiKey,
) -> Result<(), sqlx::Error> {
    let api_key_hash_bytes = api_key.hash();
    let api_key_hash_base64 = STANDARD_NO_PAD.encode(api_key_hash_bytes);

    query!(
        "DELETE FROM api_keys WHERE api_key_hash_base64 = ?",
        api_key_hash_base64,
    )
    .execute(pool)
    .await?;

    Ok(())
}
