use dotenv::dotenv;
use playswitch::switch_spotify_to_apple;
use tokio;
use utils::check_env;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Check if environment variables are set
    if !check_env() {
        eprintln!("Environment variables not set correctly!");
        std::process::exit(1)
    }

    // Start switcher from spotify to apple
    switch_spotify_to_apple().await
}
