use crate::utils::{get_url_header, Playlist};
use isrc::Isrc;
use spotify_rs::{
    auth::{AuthCodeFlow, NoVerifier, Token},
    client::Client,
    model::PlayableItem,
    AuthCodeClient, RedirectUrl,
};
use std::env;
use std::error::Error;
use std::net::TcpListener;
use std::time::Instant;

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

pub async fn get_saved_tracks() -> Result<Playlist, Box<dyn Error>> {
    let mut spot = authenticate().await?;

    // isrcs of songs that have them
    let mut saved: Vec<Isrc> = vec![];
    // spotify ids of songs without isrcs
    let mut missing: Vec<String> = vec![];
    let mut offset = 0;
    let now = Instant::now();
    loop {
        println! {"Fetching tracks {} to {}", offset, offset+50};
        let current = spot.saved_tracks().offset(offset).limit(50).get().await?;
        for item in current.items {
            match item.track.external_ids.isrc {
                Some(i) => {
                    let code: Result<Isrc, _> = i.clone().try_into();
                    match code {
                        Ok(code) => {
                            saved.push(code);
                        }
                        _ => {
                            println! {"Non-conformant (probably a new overflow CC): {}", i};
                            missing.push(item.track.id);
                        }
                    }
                }
                None => {
                    missing.push(item.track.id);
                }
            }
        }
        if offset + 50 < current.total {
            offset += 50;
        } else {
            break;
        }
    }

    println! {"time taken: {}s", now.elapsed().as_secs()};
    println! {"saved tracks with isrcs: {}", saved.len()};
    println! {"saved tracks missing isrcs: {}", missing.len()};

    Ok(Playlist {
        name: "Liked Tracks".into(),
        tracks: saved,
    })
}

pub async fn get_playlist(playlist_id: &str) -> Result<Playlist, Box<dyn Error>> {
    let mut spot = authenticate().await?;
    let mut playlist: Vec<Isrc> = vec![];
    let name: String = spot.playlist(playlist_id).get().await?.name;
    // spotify ids of songs without isrcs
    let mut missing: Vec<String> = vec![];
    let mut offset = 0;
    let now = Instant::now();
    loop {
        println! {"Fetching tracks {} to {}", offset, offset+50};
        let current = spot
            .playlist_items(playlist_id)
            .offset(offset)
            .limit(50)
            .get()
            .await?;
        for item in current.items {
            match item.track {
                PlayableItem::Track(item) => match item.external_ids.isrc {
                    Some(i) => {
                        let code: Result<Isrc, _> = i.clone().try_into();
                        match code {
                            Ok(code) => {
                                playlist.push(code);
                            }
                            _ => {
                                println! {"Non-conformant (probably a new overflow CC): {}", i};
                                missing.push(item.id);
                            }
                        }
                    }
                    None => {
                        missing.push(item.id);
                    }
                },
                PlayableItem::Episode(ep) => {
                    println! {"Episodes/podcasts do not have isrcs so cannot use this tool; {}", ep.id};
                }
            }
        }
        if offset + 50 < current.total {
            offset += 50;
        } else {
            break;
        }
    }

    println! {"time taken: {}s", now.elapsed().as_secs()};
    println! {"tracks with isrcs: {}", playlist.len()};
    println! {"tracks missing isrcs: {}", missing.len()};

    Ok(Playlist {
        name,
        tracks: playlist,
    })
}
