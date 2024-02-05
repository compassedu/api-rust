use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;

pub async fn get_staff(
    cookies: String,
    user_id: i32,
    school_id: &str,
) -> Result<Vec<GetStaffResponse>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/User.svc/GetAllStaff",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let body = GetAllStaffRequest {
        target_user_id: user_id,
        id: user_id,
    };
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .json(&body)
        .send()
        .await?;
    let res = req.json::<GetAllStaffRes>().await?;
    let mut staff: Vec<GetStaffResponse> = Vec::new();
    for ele in res.d {
        let member = GetStaffResponse {
            id: ele.id,
            name: ele.name,
            import_id: ele.import_id,
            first_name: ele.first_name,
            last_name: ele.last_name,
            start: ele.start,
            finish: ele.finish,
            picture: ele.picture,
            profile_picture: ele.profile_picture,
            user_status: ele.user_status,
            base_role: ele.base_role,
            campus_id: ele.campus_id,
            ce: ele.ce,
            display_code: ele.display_code,
            do_not_contact: ele.do_not_contact,
            f: ele.f,
            government_code_01: ele.government_code_01,
            government_code_02: ele.government_code_02,
            has_registered_device: ele.has_registered_device,
            mobile_number: ele.mobile_number,
            name_first_pref_last_id_form: ele.name_first_pref_last_id_form,
            name_pref_first: ele.name_pref_first,
            name_pref_last_id: ele.name_pref_last_id,
        };
        staff.push(member);
    }
    // print!("{:?}", req.text().await?);
    Ok(staff)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAllStaffRequest {
    #[serde(rename = "targetUserId")]
    target_user_id: i32,
    id: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStaffResponse {
    pub id: i32,
    pub name: String,
    pub import_id: String,
    pub first_name: String,
    pub last_name: String,
    pub start: String,
    pub finish: Option<String>,
    pub picture: Option<String>,
    pub profile_picture: String,
    pub user_status: i32,
    pub base_role: i32,
    pub campus_id: Option<i32>,
    pub ce: String,
    pub display_code: String,
    pub do_not_contact: bool,
    pub f: String,
    pub government_code_01: String,
    pub government_code_02: String,
    pub has_registered_device: bool,
    pub mobile_number: String,
    pub name_first_pref_last_id_form: String,
    pub name_pref_first: String,
    pub name_pref_last_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAllStaffRes {
    d: Vec<User>,
}
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    #[serde(rename = "baseRole")]
    base_role: i32,
    #[serde(rename = "campusId")]
    campus_id: Option<i32>,
    ce: String,
    #[serde(rename = "displayCode")]
    display_code: String,
    #[serde(rename = "doNotContact")]
    do_not_contact: bool,
    f: String,
    finish: Option<String>,
    #[serde(rename = "fn")]
    first_name: String,
    #[serde(rename = "govtCode1")]
    government_code_01: String,
    #[serde(rename = "govtCode2")]
    government_code_02: String,
    #[serde(rename = "hasRegisteredDevice")]
    has_registered_device: bool,
    #[serde(rename = "ii")]
    import_id: String,
    #[serde(rename = "ln")]
    last_name: String,
    #[serde(rename = "mobileNumber")]
    mobile_number: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "nameFirstPrefLastIdForm")]
    name_first_pref_last_id_form: String,
    #[serde(rename = "namePrefFirst")]
    name_pref_first: String,
    #[serde(rename = "namePrefLastId")]
    name_pref_last_id: String,
    #[serde(rename = "p")]
    picture: Option<String>,
    #[serde(rename = "pv")]
    profile_picture: String,
    start: String,
    #[serde(rename = "userStatus")]
    user_status: i32,
}
