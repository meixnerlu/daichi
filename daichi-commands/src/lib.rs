use daichi::*;

mod about;
mod counter;
mod ficolo;
mod help;
mod leaderboard;
mod owner;
mod vc_highlight;

pub fn get_commands() -> Vec<poise::Command<DcData, Error>> {
    vec![
        counter::counter(),
        ficolo::ficolo(),
        vc_highlight::vc_highlight(),
        leaderboard::leaderboard(),
        about::about(),
        owner::owner(),
        help::help(),
    ]
}
