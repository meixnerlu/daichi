use daichi::*;
use daichi_handlers::on_error_user;
use daichi_models::guildsetup::GuildSetup;
use daichi_utils::{button_selects::bool_select, checks::check_guild};
use poise::command;

/// Creates a new leaderboard message
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn replace(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    if !bool_select(
        ctx,
        guild_id,
        "Are you sure you want to create the leaderboard in this channel?",
    )
    .await?
    {
        return Ok(());
    }

    let msg = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            serenity::CreateMessage::new().content("Leaderboard:\n"),
        )
        .await?;

    GuildSetup::change_message_id(guild_id, msg.channel_id, msg.id).await?;

    Ok(())
}
