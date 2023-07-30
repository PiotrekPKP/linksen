use crate::types::{MusicClient, PlaylistItem, PlaylistItemId};
use async_trait::async_trait;
use colored::Colorize;
use rspotify::{
    model::{PlayableItem, PlaylistId},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};
use std::io::Write;
pub struct Spotify {
    client: ClientCredsSpotify,
}

impl Spotify {
    pub fn new() -> Spotify {
        let creds = Credentials::from_env().unwrap();
        let spotify = ClientCredsSpotify::new(creds);

        Spotify { client: spotify }
    }

    pub async fn authenticate(&self) {
        self.client.request_token().await.unwrap();
    }
}

#[async_trait]
impl MusicClient for Spotify {
    async fn get_playlist_items(&self, url: &str) -> Vec<PlaylistItem> {
        print!("{}", "Loading playlist... ".yellow());

        let playlist_id = extract_playlist_id(url).unwrap();
        let playlist_id = PlaylistId::from_id(playlist_id).unwrap();

        let playlist = self
            .client
            .playlist_items_manual(playlist_id, None, None, None, None)
            .await;

        let playlist_items = playlist
            .iter()
            .flat_map(|page| {
                return page.items.iter().map(|playlist_item| {
                    if let Some(playable) = &playlist_item.track {
                        match playable {
                            PlayableItem::Track(track) => {
                                let id = if let Some(track_id) = &track.id {
                                    PlaylistItemId::Spotify(track_id.to_string())
                                } else {
                                    return None;
                                };

                                let name = track.name.clone();
                                let artists = track
                                    .artists
                                    .iter()
                                    .map(|artist| artist.name.clone())
                                    .collect::<Vec<_>>()
                                    .join(", ");

                                let handle = format!("{} - {}", name, artists);

                                return Some(PlaylistItem {
                                    id,
                                    name,
                                    artists,
                                    handle,
                                });
                            }
                            PlayableItem::Episode(_) => None,
                        }
                    } else {
                        return None;
                    }
                });
            })
            .flat_map(|playlist_item| playlist_item)
            .collect::<Vec<_>>();

        println!("{}", "Playlist loaded!".green());

        return playlist_items;
    }

    async fn parse_playlist_items(&self, playlist_items: Vec<PlaylistItem>) -> Vec<PlaylistItem> {
        playlist_items
    }

    async fn search(&self, query: &String) -> Option<String> {
        unimplemented!()
    }

    async fn create_playlist(&self, playlist_items: &Vec<PlaylistItem>) {
        unimplemented!()
    }
}

fn extract_playlist_id(url: &str) -> Option<&str> {
    let prefix = "https://open.spotify.com/playlist/";
    let query_param_prefix = "?";

    if let Some(start_index) = url.find(prefix) {
        let rest_of_string = &url[start_index + prefix.len()..];

        if let Some(end_index) = rest_of_string.find(query_param_prefix) {
            return Some(&rest_of_string[..end_index]);
        } else {
            return Some(rest_of_string);
        }
    }

    return None;
}
