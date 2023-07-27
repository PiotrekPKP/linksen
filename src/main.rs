mod spotify;
mod types;
mod youtube;

use clap::{Parser, ValueEnum};
use types::MusicClient;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long)]
    url: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    SpotifyPlaylistToYoutube,
    YoutubePlaylistToSpotify,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::SpotifyPlaylistToYoutube => {
            let url = cli.url.unwrap();

            let spotify = spotify::Spotify::new();
            spotify.authenticate().await;

            let playlist_items = spotify.get_playlist_items(&url).await;

            let youtube = youtube::Youtube::new();
            let playlist_items = youtube.parse_playlist_items(playlist_items).await;

            dbg!(&playlist_items);
        }
        Mode::YoutubePlaylistToSpotify => {
            unimplemented!("youtube-playlist-to-spotify mode is not implemented yet")
        }
    }
}
