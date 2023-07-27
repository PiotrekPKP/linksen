use async_trait::async_trait;
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

use crate::types::{MusicClient, MusicClientType, PlaylistItem};

#[derive(Serialize, Deserialize)]
struct YoutubeResponseItemId {
    #[serde(rename = "videoId")]
    video_id: String,
}

#[derive(Serialize, Deserialize)]
struct YoutubeResponseItem {
    id: YoutubeResponseItemId,
}

#[derive(Serialize, Deserialize)]
struct YoutubeResponse {
    items: Vec<YoutubeResponseItem>,
}

pub struct Youtube {
    api_key: String,
}

impl Youtube {
    pub fn new() -> Self {
        let api_key = dotenv!("YOUTUBE_API_KEY").to_string();
        return Self { api_key };
    }
}

#[async_trait]
impl MusicClient for Youtube {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem> {
        unimplemented!()
    }

    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem> {
        let mut new_playlist_items = vec![];

        for playlist_item in playlist_items.iter() {
            if let MusicClientType::Youtube = playlist_item.client_type {
                new_playlist_items.push(playlist_item.clone());
            } else {
                let url = format!(
                    "https://www.googleapis.com/youtube/v3/search?part=snippet&maxResults=1&q={}&key={}",
                    playlist_item.handle, self.api_key
                );

                let youtube_response = reqwest::get(&url)
                    .await
                    .unwrap()
                    .json::<YoutubeResponse>()
                    .await
                    .unwrap();

                let video_id = youtube_response.items[0].id.video_id.clone();

                let new_playlist_item = PlaylistItem {
                    client_type: MusicClientType::Youtube,
                    id: video_id,
                    name: playlist_item.name.clone(),
                    artists: playlist_item.artists.clone(),
                    handle: playlist_item.handle.clone(),
                };

                new_playlist_items.push(new_playlist_item);
            }
        }

        return new_playlist_items;
    }
}
