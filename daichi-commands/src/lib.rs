use daichi::*;

mod about;
mod help;
mod leaderboard;
mod owner;
mod voice_highlights;

pub fn get_commands() -> Vec<poise::Command<DcData, Error>> {
    vec![
        voice_highlights::voice_highlights(),
        leaderboard::leaderboard(),
        about::about(),
        owner::owner(),
        help::help(),
    ]
}
