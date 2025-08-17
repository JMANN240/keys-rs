use std::{collections::HashMap, path::PathBuf};

use clap::{Args, Parser, Subcommand};
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
            let pre_vars = std::env::vars().collect::<HashMap<String, String>>();
            dotenvy::from_path_override(path).unwrap();
            let post_vars = std::env::vars();

            let file_vars = post_vars.filter_map(|(post_key, post_value)| {
                match pre_vars.get(&post_key) {
                    Some(pre_value) => {
                        if pre_value != &post_value {
                            Some((post_key, post_value))
                        } else {
                            None
                        }
                    },
                    None => {
                        Some((post_key, post_value))
                    }
                }
            }).collect::<HashMap<String, String>>();

            for (key, value) in file_vars.iter() {
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
