use std::{fs, process::exit};

use crate::types::{
    MusicClient, PlaylistItem, PlaylistItemId, YoutubeResponse,
    YoutubeResponseItemSectionRendererContent, YoutubeResponseSectionListRendererContent,
};
use async_trait::async_trait;
use colored::Colorize;
use dotenv_codegen::dotenv;
use google_youtube3::{
    api::{
        Playlist, PlaylistItem as PlaylistItemAPI, PlaylistItemSnippet, PlaylistSnippet, ResourceId,
    },
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
    YouTube as YouTubeAPI,
};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::io::Write;

pub struct Youtube {
    hub: Option<YouTubeAPI<HttpsConnector<HttpConnector>>>,
}

impl Youtube {
    pub fn new() -> Self {
        return Self { hub: None };
    }

    pub async fn init_api_hub(&mut self) {
        let client_id = dotenv!("GOOGLE_CLIENT_ID");
        let client_secret = dotenv!("GOOGLE_CLIENT_SECRET");
        let auth_uri = dotenv!("GOOGLE_AUTH_URI");
        let token_uri = dotenv!("GOOGLE_TOKEN_URI");

        let secret = ApplicationSecret {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_uri: auth_uri.into(),
            token_uri: token_uri.into(),
            redirect_uris: vec![],
            ..Default::default()
        };

        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.to_str().unwrap();

        fs::create_dir_all(format!("{}{}", data_dir, "/linksen")).unwrap();
        let cache_path = format!("{}{}", data_dir, "/linksen/linksen.cache");

        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(cache_path)
                .build()
                .await
                .unwrap();

        let hub = YouTubeAPI::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        );

        self.hub = Some(hub);
    }
}

#[async_trait]
impl MusicClient for Youtube {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem> {
        unimplemented!()
    }

    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem> {
        println!("{}", "Transforming playlist...".yellow());

        let pb = ProgressBar::new(playlist_items.len() as u64);
        pb.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        let mut new_playlist_items = vec![];

        for playlist_item in playlist_items.iter() {
            pb.set_message(format!("{}", playlist_item.name));

            if let PlaylistItemId::YouTube(_) = playlist_item.id {
                new_playlist_items.push(playlist_item.clone());
            } else {
                let video_id = self.search(&playlist_item.handle).await;

                if let Some(video_id) = video_id {
                    new_playlist_items.push(PlaylistItem {
                        id: PlaylistItemId::YouTube(video_id),
                        name: playlist_item.name.clone(),
                        handle: playlist_item.handle.clone(),
                        artists: playlist_item.artists.clone(),
                    })
                }
            }

            pb.inc(1);
        }

        pb.finish_with_message(format!("{}", "Transformed playlist!".green()));

        return new_playlist_items;
    }

    async fn search(&self, query: &String) -> Option<String> {
        let url = format!("https://www.youtube.com/results?search_query={}", query);

        let youtube_response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let initial_data = extract_yt_initial_data(&youtube_response).unwrap();

        let youtube_response: YoutubeResponse = serde_json::from_str(&initial_data).unwrap();

        let section_content = &youtube_response
            .contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents[0];

        let first_video_id = match section_content {
            YoutubeResponseSectionListRendererContent::ItemSectionRenderer(
                item_section_renderer,
            ) => {
                let video_renderer = &item_section_renderer.item_section_renderer.contents[0];

                match video_renderer {
                    YoutubeResponseItemSectionRendererContent::VideoRenderer(video_renderer) => {
                        Some(video_renderer.video_renderer.video_id.clone())
                    }
                    YoutubeResponseItemSectionRendererContent::OtherRenderer(_) => None,
                }
            }
            YoutubeResponseSectionListRendererContent::ContinuationItemRenderer(_) => None,
        };

        first_video_id
    }

    async fn create_playlist(&self, playlist_items: &Vec<PlaylistItem>) {
        if self.hub.is_none() {
            eprintln!(
                "{}",
                "[INTERNAL ERROR] YouTube API has not been initialized!"
                    .on_red()
                    .white()
            );

            exit(1);
        }

        let mut playlist_name = None;
        while playlist_name.is_none() {
            print!("Playlist name: ");
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim().len() == 0 {
                println!("[ERROR] Playlist name cannot be empty!");
                continue;
            }

            playlist_name = Some(input.trim().to_string());
        }

        print!("{}", "Creating playlist... ".yellow());
        let _ = std::io::stdout().flush();

        let hub = self.hub.as_ref().unwrap();

        let (_, new_playlist) = hub
            .playlists()
            .insert(Playlist {
                snippet: Some(PlaylistSnippet {
                    title: Some(playlist_name.unwrap()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .doit()
            .await
            .unwrap();

        for playlist_item in playlist_items {
            if let PlaylistItemId::YouTube(video_id) = &playlist_item.id {
                hub.playlist_items()
                    .insert(PlaylistItemAPI {
                        snippet: Some(PlaylistItemSnippet {
                            playlist_id: Some(new_playlist.id.clone().unwrap()),
                            resource_id: Some(ResourceId {
                                video_id: Some(video_id.clone()),
                                kind: Some("youtube#video".to_string()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .doit()
                    .await
                    .unwrap();
            }
        }

        println!("{}", "Created playlist!".green());
    }
}

fn extract_yt_initial_data(input: &str) -> Option<String> {
    let re = Regex::new(r#"var ytInitialData = (.*?)};"#).unwrap();

    if let Some(captured) = re.captures(input) {
        if let Some(matched_text) = captured.get(1) {
            return Some(format!("{}}}", matched_text.as_str().to_string()));
        }
    }

    None
}
