use api_rust::endpoints::{
    auth::login::authenticate_user_credentials, calendar::eventsbyuser::get_calendar_events_by_user,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let sid = "kilianssc-ie";
    let credentials: api_rust::endpoints::auth::login::AuthenticatedUserCredentials =
        authenticate_user_credentials("KOGO25", "Killian20*", "kilianssc-ie")
            .await
            .expect("Ok");
    let events = get_calendar_events_by_user(
        credentials.cookies,
        credentials.user_id,
        "2024-02-06".to_string(),
        "2024-02-06".to_string(),
        sid,
    )
    .await
    .expect("Ok");
    print!("{:?}", events);
}
