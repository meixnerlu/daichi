use daichi::*;
use poise::command;

mod join;
mod leave;
mod output;
mod play;

/// use ´/help vc_highlight´
///
/// Whenever daichi is in a voicechat he starts recording
/// Daichi will discard anything that has been said after 1 minute or the leave command
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("leave::leave", "output::output_wav", "join::join", "play::play"),
    subcommand_required
)]
pub async fn vc_highlight(_: Context<'_>) -> Result<()> {
    Ok(())
}
