use daichi::*;
use daichi_handlers::handle_play;
use daichi_handlers::on_error_user;
use poise::command;

/// Plays the last minute in the voicechannel and asks wether you want to save it
#[command(
    slash_command,
    guild_only,
    on_error = "on_error_user",
    guild_cooldown = 10,
    ephemeral
)]
pub async fn play(ctx: Context<'_>) -> Result<()> {
    handle_play(&ctx).await?;

    Ok(())
}
