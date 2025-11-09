use crate::helper::audio_codec::decrypt_url;
use serde::{Deserialize, Serialize};

pub enum SearchType {
    Album,
    Artist,
    General,
    Song,
    Playlist,
}

impl SearchType {
    pub fn value(&self) -> &str {
        match *self {
            SearchType::Album => "search.getPlaylistResults",
            SearchType::Artist => "search.getAlbumResults",
            SearchType::General => "search.getMoreResults",
            SearchType::Playlist => "search.getPlaylistResults",
            SearchType::Song => "autocomplete.get",
        }
    }

    pub fn enum_val(string_value: &str) -> Option<SearchType> {
        match string_value {
            "album" => Some(SearchType::Album),
            "artist" => Some(SearchType::Artist),
            "general" => Some(SearchType::General),
            "song" => Some(SearchType::Song),
            "playlist" => Some(SearchType::Playlist),
            _ => Some(SearchType::Song),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SearchSongQuery {
    pub search_input: Option<String>,
    pub result_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongMain {
    pub results: Option<Vec<SearchSongResult>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongResult {
    pub id: Option<String>,
    pub image: Option<String>,
    pub language: Option<String>,
    pub perma_url: Option<String>,
    pub subtitle: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub year: Option<String>,
    pub more_info: Option<SearchSongMoreInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSongMoreInfo {
    #[serde(deserialize_with = "convert_string")]
    pub encrypted_media_url: Option<String>,
    #[serde(rename = "320kbps")]
    pub is_320: Option<String>,
    pub vlink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality_link: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
}

fn convert_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    Ok(Some(decrypt_url(value.expect("null")).to_owned()))
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongDetailsQuery {
    pub id: Option<String>,
}
