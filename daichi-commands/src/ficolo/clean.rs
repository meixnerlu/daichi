use serenity::futures::StreamExt;

use super::*;

/// Sends an overview for the ficolo questions
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    required_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn clean(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let bot_id = ctx.http().get_current_application_info().await?.id;
    let (_, channel_id) = FicoloSetup::get_data(guild_id).await?;
    let mut messages = channel_id.messages_iter(ctx.http()).boxed();

    while let Some(Ok(msg)) = messages.next().await {
        if msg.author.id.to_string() != bot_id.to_string() {
            msg.delete(ctx.http()).await?;
        }
    }

    ctx.reply("All clean 👍").await?;
    Ok(())
}
