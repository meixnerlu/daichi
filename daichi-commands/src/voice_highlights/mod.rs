use daichi::*;
use poise::command;

mod join;
mod leave;
mod output;
mod play;

/// Allows you to save highlights in your Voicechannel
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("leave::leave", "output::output_wav", "join::join", "play::play"),
    subcommand_required
)]
pub async fn voice_highlights(_: Context<'_>) -> Result<()> {
    Ok(())
}
