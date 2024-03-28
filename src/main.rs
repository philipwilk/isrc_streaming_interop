mod spotify;
mod utils;
mod youtube;
use std::error::Error;

pub async fn get_action() -> Result<(), Box<dyn Error>> {
    let source = std::env::args().nth(1).expect("No source provided");
    let destination = std::env::args().nth(2).expect("No destination provided");
    let playlist_id = std::env::args().nth(3).expect("No playlist provided");

    Ok(execute(&source, &destination, &playlist_id).await?)
}

pub async fn execute(source: &str, dest: &str, id: &str) -> Result<(), Box<dyn Error>> {
    let playlist: utils::Playlist;
    match source.as_ref() {
        "spotify" => {
            if id == "liked" {
                playlist = spotify::get_saved_tracks().await?;
            } else {
                playlist = spotify::get_playlist(id).await?;
            }
        }
        x => {
            return Err(("Source ".to_string() + &x + " is not implemented!").into());
        }
    }

    let url: String;

    match dest.as_ref() {
        "youtube" => {
            let youtube_ids = youtube::playlist_to_ids(playlist.tracks).await?;
            println! {"Following tracks are missing or not tagged on youtube: {:?}", &youtube_ids.missing};
            url = youtube::create_playlist(&playlist.name, youtube_ids.found).await?;
        }
        "spotify" => {
            todo! {};
        }
        x => {
            return Err(("Destination ".to_string() + &x + " is not implemented!").into());
        }
    }

    println! {"Moved playlist {} from {} to {}.\nNew url is {}", &playlist.name, &source, &dest, &url};
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_action().await?;

    Ok(())
}
