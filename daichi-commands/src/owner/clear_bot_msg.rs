use daichi::*;
use daichi_handlers::on_error_owner;
use poise::command;

#[command(
    slash_command,
    guild_only,
    ephemeral,
    on_error = "on_error_owner",
    owners_only
)]
pub async fn clear_bot_msg(ctx: Context<'_>) -> Result<()> {
    let channel = ctx.guild_channel().await.unwrap();
    let bot_id = ctx.http().application_id().unwrap();
    ctx.reply("Cleaning up...").await?;
    if channel.is_text_based() {
        for msg in channel
            .messages(ctx, serenity::GetMessages::default())
            .await?
        {
            if msg.author.id.to_string() == bot_id.to_string() {
                msg.delete(ctx.http()).await?;
            }
        }
    }
    Ok(())
}
