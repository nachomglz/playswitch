use dotenv::dotenv;
use playswitch::switch_spotify_to_apple;
use tokio;

mod spotify;
mod apple;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Check if environment variables are set
    if !utils::check_env() {
        eprintln!("Environment variables not set correctly!");
        std::process::exit(1)
    }

    // Start switcher from spotify to apple
    switch_spotify_to_apple().await
}
