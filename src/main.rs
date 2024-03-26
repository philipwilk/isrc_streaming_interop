mod spotify;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get spotify playlist
    // let saves = spotify::get_saved_tracks().await?;

    // println! {"Total liked songs: {}", saves.len()};

    // using my gym playlist rn bc it's small
    let playlist = spotify::get_playlist("5TgNOf91pDgdWG6HprVnGf").await?;

    println! {"Total songs in playlist: {}", playlist.len()};

    // convert to isrcs

    // get yt songs

    // add to yt playlist
    Ok(())
}
