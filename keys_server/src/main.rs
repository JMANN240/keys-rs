use std::{env, fmt::Debug, fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf};

use axum::{extract::{Path, State}, routing::{get, post}, serve::Listener, Json};
use clap::{Args, Parser};
use dotenvy::dotenv;
use keys_lib::KeyValue;
use sqlx::SqlitePool;
use tokio::net::{TcpListener, UnixListener};

mod db;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    listen: Listen,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Listen {
    #[arg(short, long, group = "listen")]
    port: Option<u16>,

    #[arg(short, long, group = "listen")]
    uds: Option<PathBuf>,
}

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dotenv().unwrap();

    if let Some(port) = cli.listen.port {
        serve_with_listener(TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap()).await;
    } else if let Some(path) = cli.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        let listener = UnixListener::bind(path.clone()).unwrap();

        tokio::fs::set_permissions(path, Permissions::from_mode(0o775)).await.unwrap();

        serve_with_listener(listener).await;
    }
}

async fn serve_with_listener<L>(listener: L)
where
    L: Listener,
    L::Addr: Debug,
{
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"),
    )
    .await
    .unwrap();

    let state = AppState { pool };

    let app = axum::Router::<AppState>::new()
        .route("/{key}", get(get_key_value))
        .route("/{key}/{value}", post(set_key_value))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

pub async fn get_key_value(State(state): State<AppState>, Path(key): Path<String>) -> Json<KeyValue> {
    Json(db::get_key_value(&state.pool, key).await.unwrap())
}

pub async fn set_key_value(State(state): State<AppState>, Path((key, value)): Path<(String, String)>) -> Json<KeyValue> {
    db::upsert_key_value(&state.pool, &key, value).await.unwrap();
    Json(db::get_key_value(&state.pool, key).await.unwrap())
}