mod spotify;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get spotify playlist
    let source = spotify::get_playlist().await;

    println! {"{:?}", source};

    // convert to isrcs

    // get yt songs

    // add to yt playlist
    Ok(())
}
