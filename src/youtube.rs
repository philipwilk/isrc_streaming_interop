use google_youtube3::api::{
    Playlist, PlaylistItem, PlaylistItemContentDetails, PlaylistItemSnippet, PlaylistSnippet,
    PlaylistStatus, ResourceId,
};
use google_youtube3::hyper::client::HttpConnector;
use google_youtube3::{hyper, hyper_rustls, oauth2, YouTube};
use isrc::Isrc;
// use std::env;
use crate::utils::PlaylistResults;
use std::error::Error;

// fn get_envs() -> Result<String, Box<dyn Error>> {
//     match env::var("YOUTUBE_KEY") {
//         Ok(x) => Ok(x),
//         Err(_) => Err("Youtube api key not found".into()),
//     }
// }

async fn authenticate(
) -> Result<YouTube<google_youtube3::hyper_rustls::HttpsConnector<HttpConnector>>, Box<dyn Error>> {
    // let key = get_envs()?;

    let https = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    );

    // let secret = oauth2::ApplicationSecret {
    //     client_secret: key,
    //     client_id: "".into(),
    //     token_uri: "".into(),
    //     auth_uri: "".into(),
    //     redirect_uris: vec!["".into()],
    //     project_id: Some("".into()),
    //     client_email: Some("".into()),
    //     auth_provider_x509_cert_url: Some("".into()),
    //     client_x509_cert_url: Some("".into()),
    // };
    // let mut secret: oauth2::ApplicationSecret = Default::default();
    // secret.client_secret = key;
    // secret.project_id = Some("isrcstreamer".into());

    let secret = oauth2::read_application_secret("./client_secret.json").await?;

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokens.json")
    .hyper_client(https.clone())
    .build()
    .await?;

    let youtube = YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );
    Ok(youtube)
}

pub async fn playlist_to_ids(playlist: Vec<Isrc>) -> Result<PlaylistResults, Box<dyn Error>> {
    let youtube = authenticate().await?;
    let mut ids: PlaylistResults = PlaylistResults {
        found: vec![],
        missing: vec![],
    };

    for code in playlist {
        match code {
            Isrc::Code(code) => {
                let res = youtube
                    .search()
                    .list(&vec!["id".into()])
                    .max_results(1)
                    .q(&code)
                    .doit()
                    .await?;
                if res.1.page_info.unwrap().total_results.unwrap() != 0 {
                    let items = res.1.items.unwrap();
                    if items.len() == 0 {
                        ids.missing.push(Isrc::Code(code));
                    } else {
                        let resource = items[0].id.as_ref().unwrap();
                        let id = resource.video_id.clone().unwrap();
                        ids.found.push(id.to_owned());
                    }
                } else {
                    ids.missing.push(Isrc::Code(code));
                }
            }
        }
    }

    Ok(ids)
}

pub async fn create_playlist(
    playlist_name: &str,
    tracks: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let youtube = authenticate().await?;

    let playlist = Playlist {
        content_details: None,
        etag: None,
        id: None,
        kind: None,
        localizations: None,
        player: None,
        snippet: Some(PlaylistSnippet {
            channel_id: None,
            channel_title: None,
            default_language: None,
            description: None,
            localized: None,
            published_at: None,
            tags: None,
            thumbnail_video_id: None,
            thumbnails: None,
            title: Some(playlist_name.to_string()),
        }),
        status: Some(PlaylistStatus {
            privacy_status: Some("private".into()),
        }),
    };
    let id = youtube
        .playlists()
        .insert(playlist)
        .doit()
        .await?
        .1
        .id
        .unwrap();

    for track in tracks {
        youtube
            .playlist_items()
            .insert(PlaylistItem {
                content_details: Some(PlaylistItemContentDetails {
                    end_at: None,
                    note: None,
                    start_at: None,
                    video_id: Some(track.clone()),
                    video_published_at: None,
                }),
                etag: None,
                id: None,
                kind: None,
                snippet: Some(PlaylistItemSnippet {
                    channel_id: None,
                    channel_title: None,
                    description: None,
                    playlist_id: Some(id.clone()),
                    position: None,
                    published_at: None,
                    resource_id: Some(ResourceId {
                        channel_id: None,
                        kind: Some("youtube#video".into()),
                        playlist_id: None,
                        video_id: Some(track),
                    }),
                    thumbnails: None,
                    title: None,
                    video_owner_channel_id: None,
                    video_owner_channel_title: None,
                }),
                status: None,
            })
            .doit()
            .await?;
    }

    Ok("https://www.youtube.com/playlist?list=".to_string() + &id)
}
