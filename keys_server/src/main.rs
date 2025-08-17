use std::{env, fmt::Debug, fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf};

use aes_gcm::{Aes256Gcm, KeyInit};
use axum::{http::header::{AUTHORIZATION, CONTENT_TYPE}, serve::Listener};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;
use sqlx::SqlitePool;
use tokio::net::{TcpListener, UnixListener};
use tower_http::cors::CorsLayer;

mod api;
mod db;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve(ServeArgs),
    KeyGen,
}

#[derive(Args)]
pub struct ServeArgs {
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

    match cli.command {
        Commands::Serve(serve_args) => serve(serve_args).await,
        Commands::KeyGen => generate_key(),
    }
}

async fn serve(args: ServeArgs) {
    if let Some(port) = args.listen.port {
        serve_with_listener(TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap()).await;
    } else if let Some(path) = args.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        let listener = UnixListener::bind(path.clone()).unwrap();

        tokio::fs::set_permissions(path, Permissions::from_mode(0o775))
            .await
            .unwrap();

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
        .nest("/key_value", api::key_value::get_router())
        .nest("/api_key", api::api_key::get_router())
        .layer(CorsLayer::permissive().allow_headers([AUTHORIZATION, CONTENT_TYPE]))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

pub fn generate_key() {
    let key = Aes256Gcm::generate_key(aes_gcm::aead::OsRng);
    let base64_key = STANDARD_NO_PAD.encode(key);
    println!("Your master key is \"{base64_key}\". Store this securely, as it is used to encrypt and decrypt all values in the database.");
}
