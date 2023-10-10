mod utils;

pub async fn switch_spotify_to_apple() -> Result<(), Box<dyn std::error::Error>> {
    // get user playlists
    let token = std::env::var("SPOTIFY_TOKEN").unwrap();

    let user_id = utils::get_current_user_id().await?;

    println!("User id: {}", user_id);

    Ok(())
}

