mod spotify;
mod utils;
mod youtube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get spotify playlist
    // let saves = spotify::get_saved_tracks().await?;

    // println! {"Total liked songs: {}", saves.len()};

    // hard code using tiny playlist so i dont use all my yt api quota
    let playlist = spotify::get_playlist("1Kq3R9PDGsi6fqkxnAnIYU").await?;

    println! {"Total songs in playlist: {}", playlist.len()};
    // get yt songs
    let youtube_ids = youtube::playlist_to_ids(playlist).await?;

    println! {"found tracks with yt ids: {:#?}", youtube_ids.found};
    println! {"Missing or untagged track isrcs: {:?}", youtube_ids.missing};

    todo! {};
    // add to yt playlist
    Ok(())
}
