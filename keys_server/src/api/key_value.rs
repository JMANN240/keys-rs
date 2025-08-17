use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use keys_lib::{ApiKey, KeyValue};
use serde::Deserialize;

use crate::{
    AppState,
    db::{self, key_value::DbKeyValue},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_key_values))
        .route("/{key}", get(get_key_value).post(set_key_value).delete(delete_key_value))
}

pub async fn get_all_key_values(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<KeyValue>>, StatusCode> {
    match ApiKey::from_base64(authorization.token()) {
        Ok(api_key) => {
            let db_key_values = db::key_value::get_db_key_values(&state.pool, &api_key)
                .await
                .unwrap();

            Ok(Json(db_key_values.into_iter().map(|db_key_value| db_key_value.into()).collect()))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn get_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<KeyValue>, StatusCode> {
    match ApiKey::from_base64(authorization.token()) {
        Ok(api_key) => {
            let maybe_db_key_value = db::key_value::get_db_key_value(&state.pool, &api_key, &key)
                .await
                .unwrap();

            Ok(Json(match maybe_db_key_value {
                Some(db_key_value) => db_key_value.into(),
                None => KeyValue::new(key, None::<&str>),
            }))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
pub struct SetKeyValueRequest {
    value: String,
}

pub async fn set_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(SetKeyValueRequest { value }): Json<SetKeyValueRequest>,
) -> Result<Json<KeyValue>, StatusCode> {
    match ApiKey::from_base64(authorization.token()) {
        Ok(api_key) => {
            db::key_value::upsert_db_key_value(
                &state.pool,
                &DbKeyValue::new(&api_key, key.clone(), &value),
            )
            .await
            .unwrap();

            let maybe_db_key_value = db::key_value::get_db_key_value(&state.pool, &api_key, &key)
                .await
                .unwrap();

            Ok(Json(match maybe_db_key_value {
                Some(db_key_value) => db_key_value.into(),
                None => KeyValue::new(key, None::<&str>),
            }))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn delete_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<KeyValue>, StatusCode> {
    match ApiKey::from_base64(authorization.token()) {
        Ok(api_key) => {
            db::key_value::delete_db_key_value(&state.pool, &api_key, &key)
                .await
                .unwrap();

            let maybe_db_key_value = db::key_value::get_db_key_value(&state.pool, &api_key, &key)
                .await
                .unwrap();

            Ok(Json(match maybe_db_key_value {
                Some(db_key_value) => db_key_value.into(),
                None => KeyValue::new(key, None::<&str>),
            }))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
