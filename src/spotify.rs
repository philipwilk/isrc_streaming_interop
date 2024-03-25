use crate::utils::get_url_header;
use isrc::Isrc;
use spotify_rs::{
    auth::{AuthCodeFlow, NoVerifier, Token},
    client::Client,
    AuthCodeClient, RedirectUrl,
};
use std::env;
use std::error::Error;
use std::net::TcpListener;

struct ClientDetails {
    id: String,
    secret: String,
}

fn get_envs() -> Result<ClientDetails, Box<dyn Error>> {
    let id: String;
    let secret: String;
    match env::var("CLIENT_ID") {
        Ok(x) => id = x,
        Err(_) => return Err("Client ID not found".into()),
    };
    match env::var("CLIENT_SECRET") {
        Ok(x) => secret = x,
        Err(_) => return Err("Client secret not found".into()),
    };
    Ok(ClientDetails { id, secret })
}

async fn authenticate() -> Result<Client<Token, AuthCodeFlow, NoVerifier>, Box<dyn Error>> {
    // get client id and secret from env vars i cba rn
    let client_details = get_envs()?;

    // Listen to the redirect url to get the auth code and csrf after oauth
    let port = "9001";
    let sock = "127.0.0.1:".to_owned() + port;
    let listener = TcpListener::bind(sock)?;

    let redirect_url = RedirectUrl::new("http://localhost:".to_owned() + port)?;
    let auto_refresh = true;
    let scopes = vec!["user-library-read", "playlist-read-private"];
    let auth_code_flow = AuthCodeFlow::new(client_details.id, client_details.secret, scopes);
    let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_url, auto_refresh);

    // Open auth page in default browser
    println! {"Opening auth url in default browser"};
    open::that(url.to_string())?;

    // Capture requests to the redirect url and handle to get auth code and csrf
    let mut csrf_state: String = "".to_string();
    let mut auth_code: String = "".to_string();
    let line = get_url_header(listener)?;

    for var in line.split("&") {
        let current: Vec<&str> = var.split("=").collect();
        if current[0] == "code" {
            auth_code = current[1].to_string();
        } else if current[0] == "state" {
            csrf_state = current[1].to_string();
        }
    }

    let res = client.authenticate(auth_code, csrf_state).await;
    match res {
        Ok(x) => return Ok(x),
        Err(e) => return Err(Box::new(e)),
    }
}

pub async fn get_playlist() -> Result<Vec<Isrc>, Box<dyn Error>> {
    let mut spot = authenticate().await?;

    //let album = spot.get_current_user_profile().await.unwrap();

    //println! {"{:#?}", album};

    todo! {};
}
