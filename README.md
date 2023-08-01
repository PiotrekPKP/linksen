# linksen
Sync YouTube and Spotify data using CLI

### Usage
#### Spotify playlist to YouTube playlist
```
linksen spotify-playlist-to-youtube --url="<SPOTIFY_PLAYLIST_URL>"
```

#### YouTube playlist to Spotify playlist
```
linksen youtube-playlist-to-spotify --url="<YOUTUBE_PLAYLIST_URL>"
```

### Warning
Google has not verified this oauth app. You have to use your own API tokens for the Google Cloud app.

#### Use obtained tokens
```
linksen <MODE> --url="<PLAYLIST_URL>" --google-client-id="<GOOGLE_CLIENT_ID>" --google-client-secret="<GOOGLE_CLIENT_SECRET>"
```
