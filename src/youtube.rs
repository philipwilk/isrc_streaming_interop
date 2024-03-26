use google_youtube3::hyper::client::HttpConnector;
use google_youtube3::{hyper, hyper_rustls, oauth2, YouTube};
use isrc::Isrc;
// use std::env;
use std::error::Error;

pub struct PlaylistResults {
    pub found: Vec<String>,
    pub missing: Vec<Isrc>,
}

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
    let mut found: Vec<String> = vec![];
    let mut missing: Vec<Isrc> = vec![];

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
                    let resource = items[0].id.as_ref().unwrap();
                    let id = resource.video_id.clone().unwrap();
                    found.push(id.to_owned());
                } else {
                    missing.push(Isrc::Code(code));
                }
            }
        }
    }

    Ok(PlaylistResults { found, missing })
}
