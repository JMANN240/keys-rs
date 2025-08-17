use clap::{Parser, Subcommand};
use dotenvy::dotenv;
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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dotenv().unwrap();

    let client = KeysClient::default();

    let key_value = match cli.command {
        Commands::Get { key } => {
            client.get_value(&key).await
        }
        Commands::Set { key, value } => {
            client.set_value(&key, &value).await
        }
        Commands::Delete { key } => {
            client.delete_value(&key).await
        }
    };

    println!("{key_value:?}");
}
