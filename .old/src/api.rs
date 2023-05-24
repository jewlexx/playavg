use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: Owner,
    pub primary_color: Option<serde_json::Value>,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: Tracks,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Followers {
    pub href: Option<serde_json::Value>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub display_name: Option<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub owner_type: OwnerType,
    pub uri: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracks {
    pub href: String,
    pub items: Vec<Item>,
    pub limit: i64,
    pub next: Option<serde_json::Value>,
    pub offset: i64,
    pub previous: Option<serde_json::Value>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub added_at: String,
    pub added_by: Owner,
    pub is_local: bool,
    pub primary_color: Option<serde_json::Value>,
    pub track: Track,
    pub video_thumbnail: VideoThumbnail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Owner>,
    pub available_markets: Vec<String>,
    pub disc_number: i64,
    pub duration_ms: u128,
    pub episode: bool,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: i64,
    pub preview_url: Option<String>,
    pub track: bool,
    pub track_number: i64,
    #[serde(rename = "type")]
    pub track_type: TrackType,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub album_type: AlbumTypeEnum,
    pub artists: Vec<Owner>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: ReleaseDatePrecision,
    pub total_tracks: i64,
    #[serde(rename = "type")]
    pub purple_type: AlbumTypeEnum,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoThumbnail {
    pub url: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OwnerType {
    #[serde(rename = "artist")]
    Artist,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlbumTypeEnum {
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "single")]
    Single,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReleaseDatePrecision {
    #[serde(rename = "day")]
    Day,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrackType {
    #[serde(rename = "track")]
    Track,
}
