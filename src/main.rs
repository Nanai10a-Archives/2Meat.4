use two_meat_rust::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    discord::init(std::env::var("DISCORD_TOKEN").expect("Error: token not found!"))
        .await
        .unwrap();
}
