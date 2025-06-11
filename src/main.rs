use clap::Parser;
use dataspace_cli::{get_asset_access, provide_data};
use serde_json;

#[derive(clap::Parser)]
#[command(name = "Your CLI Tool", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Provide data to a public URL
    ProvideData {
        #[arg(long)]
        public_url: String,
        #[arg(long)]
        asset_id: String,
        #[arg(long)]
        provider_config: String,
        #[arg(long)]
        consumer_config: String,
    },
    /// Get access to an asset
    GetAssetAccess {
        #[arg(long)]
        asset_id: String,
        #[arg(long)]
        consumer_config: String,
        #[arg(long)]
        provider_config: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ProvideData {
            public_url,
            asset_id,
            provider_config,
            consumer_config,
        } => {
            provide_data(&public_url, &asset_id, &provider_config, &consumer_config).await?;
        }

        Commands::GetAssetAccess {
            asset_id,
            consumer_config,
            provider_config,
        } => {
            let asset_access =
                get_asset_access(&asset_id, &consumer_config, &provider_config).await?;
            println!("{}", serde_json::to_string(&asset_access)?);
        }
    }
    Ok(())
}
