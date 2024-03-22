use isrc::Isrc;

pub async fn get_playlist() -> Result<Vec<Isrc>, Box<dyn Error>> {
    let redirect_url = RedirectUrl::new("redirect_url".to_owned())?;
    let auto_refresh = true;
    let scopes = vec!["user-library-read", "playlist-read-private"];
    let auth_code_flow = AuthCodeFlow::new("client_id", "client_secret", scopes);

    // Redirect the user to this URL to get the auth code and CSRF token
    let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_url, auto_refresh);
}
