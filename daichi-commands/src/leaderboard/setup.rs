use daichi::*;
use daichi_handlers::on_error_user;
use daichi_models::{guildsetup::GuildSetup, mongo_crud::MongoCrud};
use daichi_utils::{
    button_selects::{bool_select, channel_select, role_select},
    checks::check_no_guild,
    sync_user_states::sync_user_states,
};
use poise::command;

/// Runs the leaderboard setup wizzard
#[command(
    slash_command,
    guild_only,
    check = "check_no_guild",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn setup(ctx: Context<'_>) -> Result<()> {
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

    let role_to_watch = role_select(ctx, guild_id).await?;

    let afk_channel = channel_select(ctx, guild_id).await?;

    let msg = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            serenity::CreateMessage::new().content("Leaderboard:\n"),
        )
        .await?;

    if GuildSetup::new(
        guild_id,
        ctx.channel_id(),
        role_to_watch,
        afk_channel,
        msg.id,
    )
    .insert()
    .await
    .is_err()
    {
        ctx.reply("Your server is already registerd. Use \"/setup delete\" to remove your server")
            .await?;
    }

    sync_user_states(ctx.serenity_context(), vec![guild_id]).await?;

    Ok(())
}
