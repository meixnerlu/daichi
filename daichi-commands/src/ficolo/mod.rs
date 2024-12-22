use daichi::*;
use poise::command;

mod setup;

/// use ´/help ficolo´
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("setup::setup"),
    subcommand_required
)]
pub async fn ficolo(_: Context<'_>) -> Result<()> {
    Ok(())
}
