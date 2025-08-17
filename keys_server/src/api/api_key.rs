use axum::{
    Json, Router,
    extract::State,
    routing::post,
};
use keys_lib::ApiKey;
use rand::rng;
use serde::{Deserialize, Serialize};

use crate::{
    db::{self, api_key::DbApiKey}, AppState
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_api_key))
}

#[derive(Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    api_key: String,
}

pub async fn create_api_key(
    State(state): State<AppState>,
) -> Json<CreateApiKeyResponse> {
    let api_key = ApiKey::random(&mut rng());

    let db_api_key = DbApiKey::new(&api_key);

    db::api_key::insert_db_api_key(&state.pool, &db_api_key)
        .await
        .unwrap();

    Json(CreateApiKeyResponse { api_key: api_key.to_base64() })
}
