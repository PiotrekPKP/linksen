use crate::types::{MusicClient, PlaylistItem, PlaylistItemId};
use async_trait::async_trait;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rspotify::{
    model::{PlayableItem, PlaylistId, SearchResult},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};

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

            if let PlaylistItemId::Spotify(_) = playlist_item.id {
                new_playlist_items.push(playlist_item.clone());
            } else {
                let video_id = self.search(&playlist_item.handle).await;

                if let Some(video_id) = video_id {
                    new_playlist_items.push(PlaylistItem {
                        id: PlaylistItemId::Spotify(video_id),
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
        let search_result = self
            .client
            .search(
                query,
                rspotify::model::SearchType::Track,
                None,
                None,
                Some(1),
                None,
            )
            .await;

        if let Ok(search_result) = search_result {
            return match search_result {
                SearchResult::Tracks(tracks_page) => {
                    if let Some(item) = tracks_page.items.first() {
                        if let Some(id) = &item.id {
                            Some(id.to_string().replace("spotify:track:", ""))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }

        return None;
    }

    async fn create_playlist(&self, _playlist_items: &Vec<PlaylistItem>) {
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
