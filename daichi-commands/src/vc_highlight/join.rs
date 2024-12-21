use daichi::*;
use daichi_handlers::on_error_user;
use daichi_handlers::*;
use poise::command;

/// Daichi will join the channel you specify
#[command(slash_command, guild_only, ephemeral, on_error = "on_error_user")]
pub async fn join(
    ctx: Context<'_>,
    #[channel_types("Voice")] channel: serenity::GuildChannel,
) -> Result<()> {
    if channel.bitrate.is_none() {
        ctx.reply("This is not a Voicechannel").await?;
        return Ok(());
    }

    handle_join(&ctx, channel.id).await?;

    Ok(())
}
