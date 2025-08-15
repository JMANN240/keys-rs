use keys_lib::KeyValue;
use sqlx::{SqlitePool, query, query_as};

pub struct DbKeyValue {
    key: String,
    value: String,
}

impl DbKeyValue {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

pub async fn upsert_key_value<K: AsRef<str>, V: AsRef<str>>(
    pool: &SqlitePool,
    key: K,
    value: V,
) -> Result<(), sqlx::Error> {
    let key_ref = key.as_ref();
    let value_ref = value.as_ref();

    query!("INSERT INTO key_values VALUES (?, ?)", key_ref, value_ref,)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_key_value<K: AsRef<str>>(
    pool: &SqlitePool,
    key: K,
) -> Result<KeyValue, sqlx::Error> {
    let key_ref = key.as_ref();

    let maybe_db_key_value = query_as!(
        DbKeyValue,
        "SELECT * FROM key_values WHERE key = ?",
        key_ref,
    )
    .fetch_optional(pool)
    .await?;

    Ok(match maybe_db_key_value {
        Some(db_key_value) => KeyValue::new(db_key_value.get_key(), Some(db_key_value.get_value())),
        None => KeyValue::new(key_ref.to_string(), None::<&str>),
    })
}
