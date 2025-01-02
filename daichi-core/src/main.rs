use daichi::*;
use daichi_commands::get_commands;
use daichi_handlers::event_handler;
use intents::get_intents;
use setup::setup;
use songbird::SerenityInit;

mod intents;
mod setup;

#[tokio::main]
async fn main() {
    setup().await;

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
