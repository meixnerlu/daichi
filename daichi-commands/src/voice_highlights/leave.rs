use daichi::*;
use daichi_handlers::handle_leave;
use daichi_handlers::on_error_user;
use poise::command;

/// Makes the bot leave the Voicechannel it is currently in
#[command(slash_command, guild_only, ephemeral, on_error = "on_error_user")]
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    handle_leave(&ctx).await?;

    Ok(())
}
