mod spotify;
mod apple;

fn main() {

    let token = spotify::get_spotify_token();

    println!("the token is {} ", token);
}
