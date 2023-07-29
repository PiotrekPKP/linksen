use crate::types::{
    MusicClient, PlaylistItem, PlaylistItemId, YoutubeResponse,
    YoutubeResponseItemSectionRendererContent, YoutubeResponseSectionListRendererContent,
};
use async_trait::async_trait;
use colored::Colorize;
use regex::Regex;

pub struct Youtube;

impl Youtube {
    pub fn new() -> Self {
        return Self;
    }

    pub async fn search(&self, query: &String) -> Option<String> {
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
}

#[async_trait]
impl MusicClient for Youtube {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem> {
        unimplemented!()
    }

    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem> {
        println!("{}", "Transforming playlist...".yellow());

        let mut new_playlist_items = vec![];

        for playlist_item in playlist_items.iter() {
            println!("- changing \"{}\"...", playlist_item.handle);

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
        }

        println!("{}", "Transformed playlist!".green());

        return new_playlist_items;
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
