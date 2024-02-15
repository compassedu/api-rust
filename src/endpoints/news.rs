use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;
/// Retrieves the news feed.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `school_id` - The ID of the school.
pub async fn get_news_feed(
    cookies: String,
    school_id: &str,
) -> Result<Vec<NewsItem>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/NewsFeed.svc/GetMyNewsFeed",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .send()
        .await?;
    let res = req.json::<GetMyNewsFeedRes>().await?;
    let mut items: Vec<NewsItem> = Vec::new();
    for item in res.d {
        let mut attachments: Vec<Attachment> = Vec::new();
        for attachment in item.attachments {
            let e = Attachment {
                id: attachment.id,
                file_type: attachment.file_type,
                is_image: attachment.is_image,
                name: attachment.name,
                original_file_name: attachment.original_file_name,
                ui_link: attachment.ui_link,
                url: attachment.url,
            };
            attachments.push(e);
        }
        let content = match item.content2 {
            Some(c) => item.content1 + &c,
            None => item.content1.clone(),
        };
        let e = NewsItem {
            id: item.news_item_id,
            title: item.title,
            created_at: item.post_date_time,
            communication_type: item.communication_type,
            content,
            attachments,
            created_by_admin: item.created_by_admin,
            finish: item.finish,
            priority: item.priority,
            author_id: item.user_id,
            author_image_url: item.user_image_url,
            username: item.username,
        };
        items.push(e);
    }
    Ok(items)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetMyNewsFeedRes {
    d: Vec<NewsItemRes>,
}
#[derive(Serialize, Deserialize, Debug)]
struct NewsItemAttachment {
    #[serde(rename = "AssetId")]
    id: i32,
    #[serde(rename = "FileAssetType")]
    file_type: i32,
    #[serde(rename = "IsImage")]
    is_image: bool,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OriginalFileName")]
    original_file_name: String,
    #[serde(rename = "UiLink")]
    ui_link: String,
    #[serde(rename = "Url")]
    url: Option<String>,
}
/// Represents an attachment associated with a news item.
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    /// The ID of the attachment.
    pub id: i32,
    /// The type of the file.
    pub file_type: i32,
    /// Indicates whether the attachment is an image.
    pub is_image: bool,
    /// The name of the attachment.
    pub name: String,
    /// The original file name of the attachment.
    pub original_file_name: String,
    /// The UI link of the attachment.
    pub ui_link: String,
    /// The URL of the attachment, if available.
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct NewsItemRes {
    #[serde(rename = "Attachments")]
    attachments: Vec<NewsItemAttachment>,
    #[serde(rename = "CommunicationType")]
    communication_type: i32,
    #[serde(rename = "Content1")]
    content1: String,
    #[serde(rename = "Content2")]
    content2: Option<String>,
    #[serde(rename = "CreatedByAdmin")]
    created_by_admin: bool,
    #[serde(rename = "Finish")]
    finish: String,
    #[serde(rename = "NewsItemId")]
    news_item_id: String,
    #[serde(rename = "PostDateTime")]
    post_date_time: String,
    #[serde(rename = "Priority")]
    priority: bool,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "UserId")]
    user_id: i32,
    #[serde(rename = "UserImageUrl")]
    user_image_url: String,
    #[serde(rename = "UserName")]
    username: String,
}
/// Represents a news item.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewsItem {
    /// The ID of the news item.
    pub id: String,
    /// The title of the news item.
    pub title: String,
    /// The creation timestamp of the news item.
    pub created_at: String,
    /// The list of attachments associated with the news item.
    pub attachments: Vec<Attachment>,
    /// The communication type of the news item.
    pub communication_type: i32,
    /// The content of the news item.
    pub content: String,
    /// Indicates whether the news item was created by an admin.
    pub created_by_admin: bool,
    /// The finish timestamp of the news item.
    pub finish: String,
    /// Indicates the priority of the news item.
    pub priority: bool,
    /// The ID of the author of the news item.
    pub author_id: i32,
    /// The URL of the author's image.
    pub author_image_url: String,
    /// The username of the author.
    pub username: String,
}
