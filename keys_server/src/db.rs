use keys_lib::{KeyStore, KeyValue};
use sqlx::{SqlitePool, query, query_as};

pub struct DbKeyValue {
    store: String,
    key: String,
    value: String,
}

impl DbKeyValue {
    pub fn get_store(&self) -> &str {
        &self.store
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

pub async fn get_key_value<S: AsRef<str>, K: AsRef<str>>(
    pool: &SqlitePool,
    store: S,
    key: K,
) -> Result<KeyValue, sqlx::Error> {
    let store_ref = store.as_ref();
    let key_ref = key.as_ref();

    let maybe_db_key_value = query_as!(
        DbKeyValue,
        "SELECT * FROM key_values WHERE store = ? AND key = ?",
        store_ref,
        key_ref,
    )
    .fetch_optional(pool)
    .await?;

    Ok(match maybe_db_key_value {
        Some(db_key_value) => KeyValue::new(
            KeyStore::new(db_key_value.get_store()),
            db_key_value.get_key(),
            Some(db_key_value.get_value()),
        ),
        None => KeyValue::new(KeyStore::new(store_ref), key_ref.to_string(), None::<&str>),
    })
}

pub async fn upsert_key_value(pool: &SqlitePool, key_value: KeyValue) -> Result<(), sqlx::Error> {
    let store = key_value.get_store().get_store();
    let key = key_value.get_key();
    let value = key_value.get_value();

    query!(
        "INSERT INTO key_values VALUES (?, ?, ?) ON CONFLICT (store, key) DO UPDATE SET value = ?",
        store,
        key,
        value,
        value
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_key_value<S: AsRef<str>, K: AsRef<str>>(
    pool: &SqlitePool,
    store: S,
    key: K,
) -> Result<(), sqlx::Error> {
    let store_ref = store.as_ref();
    let key_ref = key.as_ref();

    query!(
        "DELETE FROM key_values WHERE store = ? and key = ?",
        store_ref,
        key_ref
    )
    .execute(pool)
    .await?;

    Ok(())
}
