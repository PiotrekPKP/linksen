use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum MusicClientType {
    Youtube,
    Spotify,
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub client_type: MusicClientType,
    pub id: String,
    pub name: String,
    pub artists: String,
    pub handle: String,
}

#[async_trait]
pub trait MusicClient {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem>;
    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem>;
}
