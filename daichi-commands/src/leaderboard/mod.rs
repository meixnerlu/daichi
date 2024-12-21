use daichi::*;
use poise::command;

mod delete;
mod replace;
mod role_button;
mod setup;

/// use `/help leaderboard`
///
/// When set up it will track when users join and leave voicechannels
/// This is used to create a leaderboard that displays the time of 10 users
/// The users are the once with the most time spend in the channels
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands(
        "delete::delete",
        "role_button::role_button",
        "setup::setup",
        "replace::replace"
    ),
    subcommand_required
)]
pub async fn leaderboard(_: Context<'_>) -> Result<()> {
    Ok(())
}
