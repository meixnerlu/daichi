use daichi::*;
use poise::command;

mod delete;
mod role_button;
mod setup;

/// Commands to manage the Voicechat leaderboard
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("delete::delete", "role_button::role_button", "setup::setup"),
    subcommand_required
)]
pub async fn leaderboard(_: Context<'_>) -> Result<()> {
    Ok(())
}
