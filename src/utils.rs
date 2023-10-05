
#[derive(Debug)]
pub struct TokenPair {
    pub spotify_token: String,
    pub apple_token: String
}

pub fn check_env() -> bool {
    return (std::env::var("SPOTIFY_CLIENT_ID").is_ok())
        && (std::env::var("SPOTIFY_CLIENT_SECRET").is_ok());
}

pub fn get_tokens() -> Option<TokenPair> {
    return Some(TokenPair {
        spotify_token: "".to_owned(),
        apple_token: "".to_owned()
    })
}
