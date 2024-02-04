use api_rust::endpoints::auth::login::authenticate_user_credentials;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    authenticate_user_credentials("KOGO25", "Killian20*", "kilianssc-ie").await;
}
