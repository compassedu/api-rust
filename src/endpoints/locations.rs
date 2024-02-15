use crate::utils::consts::USER_AGENT;
use serde::{Deserialize, Serialize};
/// Retrieves all locations.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `school_id` - The ID of the school.
pub async fn get_all_locations(
    cookies: String,
    school_id: &str,
) -> Result<Vec<Location>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/ReferenceDataCache.svc/GetAllLocations?page=1",
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
        .get(url)
        .send()
        .await?;
    let res = req.json::<GetAllLocationsResponse>().await?;
    let mut locations: Vec<Location> = Vec::new();
    for location in res.d {
        let e = Location {
            id: location.id,
            archived: location.archived,
            building: location.building,
            long_name: location.long_name,
            name: location.name,
            room_name: location.room_name,
        };
        locations.push(e);
    }
    Ok(locations)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAllLocationsResponse {
    d: Vec<LocationResponse>,
}
#[derive(Serialize, Deserialize, Debug)]
struct LocationResponse {
    id: i32,
    archived: bool,
    building: Option<String>,
    #[serde(rename = "longName")]
    long_name: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "roomName")]
    room_name: String,
}
/// Represents a location.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    /// The ID of the location.
    pub id: i32,
    /// Indicates whether the location is archived.
    pub archived: bool,
    /// The building associated with the location.
    pub building: Option<String>,
    /// The long name of the location.
    pub long_name: String,
    /// The name of the location.
    pub name: String,
    /// The room name of the location.
    pub room_name: String,
}
