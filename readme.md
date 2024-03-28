# Transfer your playlists from spotify to youtube
Theoretically this can be improved upon to work for ANY streaming services where:
- the source service allows retrieval of isrc tags from tracks
- the destination services allows searching for tracks via isrc tags

From my experience, it seems that spotify/publishers on spotify does a very good job with tagging their releases with isrcs,
but on youtube it appears to be less widestream.
I think that only songs that you can see marked with a content id on youtube have the proper ISRC tags; some songs ARE on youtube but AREN'T TAGGED properly!!!

This means that there may be false negatives where a track is not found, but there CANNOT be false positives where a track is substituted with an incorrect one; an ISRC will always match the exact recording of a track.

## Issues
- Also currently idk how you could add songs to your saved tracks on yt music but that would be nice.
  Spotify supports this but I don't think ytm does.
- the yt search function is api expensive (100 credits), maybe if ytm devs <3 this enough they could add a cheaper search that retrieves the one or none track matching the ISRC.
- spotify auth isnt cached bc i cba for a PoC

## How to use
create env vars for spotify:
- CLIENT_ID with a dev app id
- CLIENT_SECRET with a dev app secret

create a youtube oauth 2.0 client id with permission to use the "YouTube Data API v3" scope in the GCP console and save as "client_secret.json"

The cli at the moment is really simple, it's just:
``cargo run -- source destination playlist_id``
For example, to transfer from spotify to youtube, you would do:
``cargo run -- spotify youtube (playlist_id)``
You can also run using 'liked' instead of the playlist id to get your liked tracks:
``cargo run -- spotify youtube liked``
