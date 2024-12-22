use std::fs;

use daichi::*;
use daichi_commands::get_commands;
use daichi_handlers::event_handler;
use daichi_models::{leaderboardsetup::LeaderboardSetup, user_dc_event::UserDcEvent};
use intents::get_intents;
use songbird::SerenityInit;

mod intents;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let _ = fs::create_dir("/tmp/daichi");
    let _ = UserDcEvent::setup_collection().await;
    let _ = LeaderboardSetup::setup_collection().await;

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN to be present");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| Box::pin(async { Ok(DcData()) }))
        .build();

    let songbird_config =
        songbird::Config::default().decode_mode(songbird::driver::DecodeMode::Decode);

    let client = serenity::ClientBuilder::new(token, get_intents())
        .framework(framework)
        .register_songbird_from_config(songbird_config)
        .await;
    client.unwrap().start().await.unwrap();
}
