mod spotify;
mod types;
mod youtube;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::io::Write;
use types::MusicClient;

#[derive(Parser)]
struct Cli {
    /// Mode to run the tool in
    #[arg(value_enum)]
    mode: Mode,

    /// Playlist URL
    #[arg(short, long)]
    url: Option<String>,

    /// Google OAuth Client ID
    #[arg(long)]
    google_client_id: Option<String>,

    /// Google OAuth Client Secret
    #[arg(long)]
    google_client_secret: Option<String>,
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
            println!("{}", "Welcome to linksen!".on_blue().black());
            println!("Mode: {}", "Spotify playlist -> YouTube playlist".blue());
            println!();

            let url = cli.url.unwrap();

            let spotify = spotify::Spotify::new();
            spotify.authenticate().await;

            let playlist_items = spotify.get_playlist_items(&url).await;

            println!();

            let mut youtube = youtube::Youtube::new();
            let playlist_items = youtube.parse_playlist_items(playlist_items).await;

            println!();
            print!("Do you want to create a playlist? [Y/n] ");
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "yes"
                || input.trim().to_lowercase() == "y"
                || input.trim() == ""
            {
                println!();
                println!("{}", "Creating playlist".on_green().black());

                youtube
                    .init_api_hub(cli.google_client_id, cli.google_client_secret)
                    .await;
                youtube.create_playlist(&playlist_items).await;
            } else {
                println!();
                println!("{}", "Playlist items".on_green().black());

                for playlist_item in playlist_items {
                    println!(
                        "{}: {}",
                        playlist_item.name.green(),
                        playlist_item.id.to_string().blue()
                    );
                }
            }
        }
        Mode::YoutubePlaylistToSpotify => {
            println!("{}", "Welcome to linksen!".on_blue().black());
            println!("Mode: {}", "YouTube playlist -> Spotify playlist".blue());
            println!();

            let url = cli.url.unwrap();

            let mut youtube = youtube::Youtube::new();
            youtube
                .init_api_hub(cli.google_client_id, cli.google_client_secret)
                .await;

            let playlist_items = youtube.get_playlist_items(&url).await;

            println!();

            let spotify = spotify::Spotify::new();
            spotify.authenticate().await;

            let playlist_items = spotify.parse_playlist_items(playlist_items).await;

            println!();
            println!();
            println!("{}", "Playlist items".on_green().black());

            for playlist_item in playlist_items {
                println!(
                    "{}: {}",
                    playlist_item.name.green(),
                    playlist_item.id.to_string().blue()
                );
            }
        }
    }
}
