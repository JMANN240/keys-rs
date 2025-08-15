use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use keys_lib::{KeyStore, KeyValue};

use crate::{AppState, db};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/{key}", get(get_key_value).delete(delete_key_value))
        .route("/{key}/{value}", post(set_key_value))
}

pub async fn get_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Json<KeyValue> {
    Json(
        db::get_key_value(&state.pool, authorization.token(), key)
            .await
            .unwrap(),
    )
}

pub async fn set_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path((key, value)): Path<(String, String)>,
) -> Json<KeyValue> {
    db::upsert_key_value(
        &state.pool,
        KeyValue::new(KeyStore::new(authorization.token()), &key, Some(value)),
    )
    .await
    .unwrap();
    Json(
        db::get_key_value(&state.pool, authorization.token(), key)
            .await
            .unwrap(),
    )
}

pub async fn delete_key_value(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Json<KeyValue> {
    db::delete_key_value(&state.pool, authorization.token(), &key)
        .await
        .unwrap();
    Json(
        db::get_key_value(&state.pool, authorization.token(), key)
            .await
            .unwrap(),
    )
}
