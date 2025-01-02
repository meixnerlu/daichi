use std::fs;

use daichi_models::{
    ficolo_question::FicoloQuestion, ficolosetup::FicoloSetup, leaderboardsetup::LeaderboardSetup,
    user_dc_event::UserDcEvent,
};

pub async fn setup() {
    let _ = dotenvy::dotenv();
    let _ = fs::create_dir("/tmp/daichi");
    let _ = UserDcEvent::setup_collection().await;
    let _ = LeaderboardSetup::setup_collection().await;
    let _ = FicoloSetup::setup_collection().await;
    let _ = FicoloQuestion::setup_collection().await;
}
