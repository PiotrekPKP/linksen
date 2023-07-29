mod spotify;
mod types;
mod youtube;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::io::Write;
use types::MusicClient;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(short, long)]
    url: Option<String>,

    #[arg(short, long)]
    query: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    SpotifyPlaylistToYoutube,
    YoutubePlaylistToSpotify,
    SearchYoutube,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::SpotifyPlaylistToYoutube => {
            println!("{}", "Welcome to linksen!".on_bright_green().bold());
            println!("Mode: {}", "Spotify playlist -> YouTube playlist".green());

            let url = cli.url.unwrap();

            let spotify = spotify::Spotify::new();
            spotify.authenticate().await;

            let playlist_items = spotify.get_playlist_items(&url).await;

            let youtube = youtube::Youtube::new();
            //let playlist_items = youtube.parse_playlist_items(playlist_items).await;

            println!();
            print!("Do you want to create a playlist? [Y/n] ");
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "Y" || input.trim() == "y" || input.trim() == "" {
                println!("{}", "Creating playlist...".yellow());
            }
        }
        Mode::SearchYoutube => {
            println!("{}", "Welcome to linksen!".on_bright_green().bold());
            println!("Mode: {}", "Search YouTube".green());

            let query = cli.query.unwrap();

            let youtube = youtube::Youtube::new();
            let search_result = youtube.search(&query).await;

            if let Some(video_id) = search_result {
                println!(
                    "Found a video! URL: {}",
                    format!("https://www.youtube.com/watch?v={}", video_id).green()
                );
            } else {
                println!("{}", "No videos found :(".red());
            }
        }
        Mode::YoutubePlaylistToSpotify => {
            unimplemented!("youtube-playlist-to-spotify mode is not implemented yet")
        }
    }
}
