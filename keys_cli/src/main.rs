use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use dotenvy::{EnvLoader, EnvSequence};
use keys_client::KeysClient;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Migrate(MigrateArgs),
}

#[derive(Args)]
struct MigrateArgs {
    #[arg(default_value = ".env")]
    path: PathBuf
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = KeysClient::default();

    match cli.command {
        Commands::Get { key } => {
            println!("{:?}", client.get_value(&key).await);
        },
        Commands::Set { key, value } => {
            println!("{:?}", client.set_value(&key, &value).await);
        },
        Commands::Delete { key } => {
            println!("{:?}", client.delete_value(&key).await);
        },
        Commands::Migrate(MigrateArgs { path }) => {
            let env = EnvLoader::with_path(path).sequence(EnvSequence::InputOnly).load().unwrap();

            for (key, value) in env.iter() {
                let key_value = client.get_value(&key).await.unwrap();

                match key_value.get_value() {
                    Some(value) => {
                        println!("Value '{value}' already exists for key '{key}', not migrating.");
                    },
                    None => {
                        client.set_value(&key, &value).await.unwrap();
                        println!("Migrated value '{value}' for key '{key}'.");
                    }
                }
            }
        }
    };
}
