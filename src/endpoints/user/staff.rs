use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;

/// Retrieves staff information.
///
/// This function sends a request to retrieve staff information from the server.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `user_id` - ID of the user.
/// * `school_id` - The ID of the school.
pub async fn get_staff(
    cookies: String,
    user_id: i32,
    school_id: &str,
) -> Result<Vec<StaffMember>, reqwest::Error> {
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
    let mut staff: Vec<StaffMember> = Vec::new();
    for ele in res.d {
        let member = StaffMember {
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
/// Represents a staff member.
#[derive(Serialize, Deserialize, Debug)]
pub struct StaffMember {
    /// The ID of the staff member.
    pub id: i32,
    /// The full name of the staff member.
    pub name: String,
    /// The import identifier of the staff member.
    pub import_id: String,
    /// The first name of the staff member.
    pub first_name: String,
    /// The last name of the staff member.
    pub last_name: String,
    /// The start date of the staff member's employment. In ISO 8601 format.
    pub start: String,
    /// The end date of the staff member's employment. In ISO 8601 format.
    pub finish: Option<String>,
    /// The URL to the picture of the staff member.
    pub picture: Option<String>,
    /// The URL to the profile picture of the staff member.
    pub profile_picture: String,
    /// The status of the staff member's user account.
    pub user_status: i32,
    /// The base role of the staff member.
    pub base_role: i32,
    /// The ID of the campus the staff member belongs to.
    pub campus_id: Option<i32>,
    ///
    pub ce: String,
    /// The display code of the staff member.
    pub display_code: String,
    /// Indicates whether the staff member should not be contacted.
    pub do_not_contact: bool,
    ///
    pub f: String,
    /// The first government code of the staff member.
    pub government_code_01: String,
    /// The second government code of the staff member.
    pub government_code_02: String,
    /// Indicates whether the staff member has a registered device.
    pub has_registered_device: bool,
    /// The mobile number of the staff member.
    pub mobile_number: String,
    /// The name_first_pref_last_id_form of the staff member.
    pub name_first_pref_last_id_form: String,
    /// The name_pref_first of the staff member.
    pub name_pref_first: String,
    /// The name_pref_last_id of the staff member.
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
