use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;
/// Creates user credentials.
///
/// # Arguments
///
/// * `username` - Your Compass Username.
/// * `password` - Your Compass Password.
/// * `school_id` - The ID of the school.
/// # Supported
///
/// ❌ 2fa not supported
///
/// ❌ SAML not supported
/// # Example
///
/// ```
/// let auth = authenticate_user_credentials("JOHNDOE", "***", "***").await?;
/// println!("{:?}", auth); // Output: AuthenticatedUserCredentials { success: true, user_id: 0, cookies: "***" }
/// ```
pub async fn authenticate_user_credentials(
    username: &str,
    password: &str,
    school_id: &str,
) -> Result<AuthenticatedUserCredentials, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/services/admin.svc/AuthenticateUserCredentials",
        school_id
    );
    let user_credentials: Credentials = Credentials {
        sessionstate: "readonly".to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };
    let req = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .build()
        .unwrap()
        .post(url.clone())
        .json(&user_credentials)
        .send()
        .await?;
    let cookies: Vec<String> = req
        .cookies()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect();
    let res = req.json::<AuthenticationResult>().await?;
    Ok(AuthenticatedUserCredentials {
        success: res.d.success,
        user_id: res.d.roles[0].user_id,
        cookies: cookies.join("; "),
        school_id: school_id.to_string(),
    })
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthenticatedUserCredentialsErr {
    success: bool,
    technical_message: String,
    msg: String,
}
/// Represents authenticated user credentials.
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedUserCredentials {
    /// Indicates whether the authentication was successful.
    pub success: bool,
    /// The ID of the authenticated user.
    pub user_id: i32,
    /// The cookies associated with the authenticated session.
    pub cookies: String,
    /// The school id associated with the authenticated session.
    pub school_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    sessionstate: String,
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthenticationResult {
    d: AuthData,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthData {
    #[serde(rename = "2FAuthRequired")]
    two_factor_auth_required: bool,
    #[serde(rename = "friendlyMessage")]
    friendly_message: String,
    success: bool,
    #[serde(rename = "technicalMessage")]
    technical_message: String,
    roles: Vec<AuthRoles>,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthRoles {
    #[serde(rename = "baseRole")]
    base_role: i32,
    #[serde(rename = "fqdn")]
    fully_qualified_domain_name: String,
    #[serde(rename = "userId")]
    user_id: i32,
}
