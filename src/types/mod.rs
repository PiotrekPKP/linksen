use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponse {
    pub contents: YoutubeResponseContents,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseContents {
    #[serde(rename = "twoColumnSearchResultsRenderer")]
    pub two_column_search_results_renderer: YoutubeResponseTwoColumnSearchResultsRenderer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseTwoColumnSearchResultsRenderer {
    #[serde(rename = "primaryContents")]
    pub primary_contents: YoutubeResponsePrimaryContents,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponsePrimaryContents {
    #[serde(rename = "sectionListRenderer")]
    pub section_list_renderer: YoutubeResponseSectionListRenderer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseSectionListRenderer {
    #[serde(rename = "contents")]
    pub contents: Vec<YoutubeResponseSectionListRendererContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum YoutubeResponseSectionListRendererContent {
    ItemSectionRenderer(YoutubeResponseSectionListRendererContentItemSectionRenderer),
    ContinuationItemRenderer(YoutubeResponseSectionListRendererContentContinuationItemRenderer),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseSectionListRendererContentItemSectionRenderer {
    #[serde(rename = "itemSectionRenderer")]
    pub item_section_renderer: YoutubeResponseItemSectionRenderer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseSectionListRendererContentContinuationItemRenderer {
    #[serde(rename = "continuationItemRenderer")]
    continuation_item_renderer: YoutubeResponseItemSectionContinuationRenderer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseItemSectionContinuationRenderer {
    trigger: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseItemSectionRenderer {
    #[serde(rename = "contents")]
    pub contents: Vec<YoutubeResponseItemSectionRendererContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum YoutubeResponseItemSectionRendererContent {
    VideoRenderer(YoutubeResponseItemSectionRendererContentVideoRenderer),
    OtherRenderer(Map<String, Value>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseItemSectionRendererContentVideoRenderer {
    #[serde(rename = "videoRenderer")]
    pub video_renderer: YoutubeResponseVideoRenderer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YoutubeResponseVideoRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Debug, Clone)]
pub enum PlaylistItemId {
    Spotify(String),
    YouTube(String),
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub id: PlaylistItemId,
    pub name: String,
    pub artists: String,
    pub handle: String,
}

#[async_trait]
pub trait MusicClient {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem>;
    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem>;
}
