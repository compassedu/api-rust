use serde::{Deserialize, Serialize};
pub async fn authenticate_user_credentials(username: &str, password: &str, school_id: &str){
    let url = format!(
        "https://{}.compass.education/services/admin.svc/AuthenticateUserCredentials",
        school_id
    );
    let user_credentials:Credentials = Credentials { sessionstate: "readonly".to_string(), username: username.to_string(), password: password.to_string() };
    let req: AuthenticationResult = reqwest::ClientBuilder::new().user_agent("Compass API Rust/ V0").cookie_store(true).build().unwrap().post(url.clone()).json(&user_credentials).send().await.expect("Ok").json::<AuthenticationResult>().await.expect("Ok");

    println!("{:?}",req);
}

#[derive(Serialize, Deserialize,Debug)]
struct Credentials {
    sessionstate: String,
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize,Debug)]
struct AuthenticationResult {
    d:AuthData,
}
#[derive(Serialize, Deserialize,Debug)]
struct AuthData{
    #[serde(rename = "2FAuthRequired")]
    two_factor_auth_required:bool,
    #[serde(rename = "friendlyMessage")]
    friendly_message:String,
    success:bool,
    #[serde(rename = "technicalMessage")]
    technical_message:String,
    roles: Vec<AuthRoles>
}
#[derive(Serialize, Deserialize,Debug)]
struct AuthRoles{
    #[serde(rename = "baseRole")]
    base_role:i32,
    fqdn:String,
    #[serde(rename = "userId")]
    user_id:i32,
}
