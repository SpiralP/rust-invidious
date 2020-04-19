use crate::{
    api::{ApiError, AuthorThumbnail, VideoThumbnail},
    error::*,
};
use serde::{Deserialize, Serialize};

pub async fn request(id: &str, params: Parameters) -> Result<Schema> {
    let client = reqwest::Client::new();
    let bytes = client
        .get(&format!("https://invidio.us/api/v1/videos/{}", id))
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
    /// ISO 3166 country code (default: "US"),
    pub region: Option<String>,
}

// Schema

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub title: String,
    pub video_id: String,
    pub video_thumbnails: Vec<VideoThumbnail>,

    pub description: String,
    pub description_html: String,
    pub published: u64,
    pub published_text: String,

    pub keywords: Vec<String>,
    pub view_count: u64,
    pub like_count: u32,
    pub dislike_count: u32,

    pub paid: bool,
    pub premium: bool,
    pub is_family_friendly: bool,
    pub allowed_regions: Vec<String>,
    pub genre: String,
    pub genre_url: String,

    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub author_thumbnails: Vec<AuthorThumbnail>,

    pub sub_count_text: String,
    pub length_seconds: u32,
    pub allow_ratings: bool,
    pub rating: f32,
    pub is_listed: bool,
    pub live_now: bool,
    pub is_upcoming: bool,
    pub premiere_timestamp: Option<u64>,

    pub hls_url: Option<String>,
    pub adaptive_formats: Vec<AdaptiveFormat>,
    pub format_streams: Vec<FormatStream>,
    pub captions: Vec<Caption>,
    pub recommended_videos: Vec<RecommendedVideo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveFormat {
    pub index: String,
    pub bitrate: String,
    pub init: String,
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub clen: String,
    pub lmt: String,

    /// a u32 as a String
    pub projection_type: String,
    pub container: String,
    pub encoding: String,
    pub quality_label: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatStream {
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub quality: String,
    pub container: String,
    pub encoding: String,
    pub quality_label: String,
    pub resolution: String,
    pub size: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caption {
    pub label: String,
    pub language_code: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedVideo {
    pub video_id: String,
    pub title: String,
    pub video_thumbnails: Vec<VideoThumbnail>,
    pub author: String,
    pub length_seconds: u32,
    pub view_count_text: String,
}
