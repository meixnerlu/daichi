use daichi::*;

mod about;
mod help;
mod leaderboard;
mod owner;
mod vc_highlight;

pub fn get_commands() -> Vec<poise::Command<DcData, Error>> {
    vec![
        vc_highlight::vc_highlight(),
        leaderboard::leaderboard(),
        about::about(),
        owner::owner(),
        help::help(),
    ]
}
