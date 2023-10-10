
main() {

  source .env
  curl -X POST "https://accounts.spotify.com/api/token" \
       -H "Content-Type: application/x-www-form-urlencoded" \
       -d "grant_type=client_credentials&client_id=${SPOTIFY_CLIENT_ID}&client_secret=${SPOTIFY_CLIENT_SECRET}" |
  jq .access_token
}

main
