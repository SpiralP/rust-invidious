use crate::{api::ApiError, error::*};
use serde::{Deserialize, Serialize};

pub async fn request(params: Parameters) -> Result<Schema> {
    let client = reqwest::Client::new();
    let bytes = client
        .get("https://invidio.us/api/v1/search")
        .query(&params)
        .send()
        .await?
        .bytes()
        .await?;

    if let Ok(error) = serde_json::from_slice::<ApiError>(&bytes) {
        Err(ErrorKind::Invidious(error.error).into())
    } else {
        Ok(serde_json::from_slice::<Schema>(&bytes)?)
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Parameters {
    pub q: Option<String>,
    pub page: Option<u32>,
    pub sort_by: Option<SortBy>,
    pub date: Option<Date>,
    pub duration: Option<Duration>,

    /// default: video
    pub r#type: Option<Type>,
    pub features: Option<Vec<Feature>>,

    /// ISO 3166 country code (default: "US"),
    pub region: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    Hd,
    Subtitles,
    CreativeCommons,
    _3d,
    Live,
    Purchased,
    _4k,
    _360,
    Location,
    Hdr,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Video,
    Playlist,
    Channel,
    All,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Duration {
    Short,
    Long,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Date {
    Hour,
    Today,
    Week,
    Month,
    Year,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    Relevance,
    Rating,
    UploadDate,
    ViewCount,
}

// Schema

pub type Schema = Vec<SchemaType>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SchemaType {
    Video(Video),
    Playlist(Playlist),
    Channel(Channel),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_thumbnails: Vec<Thumbnail>,
    pub description: String,
    pub description_html: String,
    pub view_count: u64,
    pub published: u64,
    pub published_text: String,
    pub length_seconds: u32,
    pub live_now: bool,
    pub paid: bool,
    pub premium: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub title: String,
    pub playlist_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,

    pub video_count: u32,
    pub videos: Vec<PlaylistVideo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideo {
    pub title: String,
    pub video_id: String,
    pub length_seconds: u32,
    pub video_thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,

    pub author_thumbnails: Vec<AuthorThumbnail>,
    pub sub_count: u32,
    pub video_count: u32,
    pub description: String,
    pub description_html: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorThumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub quality: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}
