use crate::utils::consts::USER_AGENT;
use serde::{Deserialize, Serialize};

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
        .post(url)
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
    building: String,
    #[serde(rename = "longName")]
    long_name: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "roomName")]
    room_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    id: i32,
    archived: bool,
    building: String,
    long_name: String,
    name: String,
    room_name: String,
}
