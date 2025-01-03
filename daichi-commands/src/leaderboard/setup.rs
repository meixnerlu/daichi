use super::*;
use daichi_models::{leaderboardsetup::LeaderboardSetup, mongo_crud::MongoCrud};
use daichi_utils::{
    button_selects::{bool_select, channel_select, role_select_opt},
    sync_user_states::sync_user_states,
};

/// Runs the leaderboard setup wizzard
#[command(
    slash_command,
    guild_only,
    check = "check_no_guild",
    on_error = "on_error_user",
    ephemeral
)]
pub async fn setup(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    if !bool_select(
        ctx,
        "Are you sure you want to create the leaderboard in this channel?",
    )
    .await?
    {
        return Ok(());
    }

    let role_to_watch = role_select_opt(ctx,
        "Do you just want to track a specific role?\n".to_string() +
        "You can later create a message for your members to get the role with \"/setup role_button\"",
    ).await?;

    let afk_channel = channel_select(ctx).await?;

    let msg = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            serenity::CreateMessage::new().content("Leaderboard:\n"),
        )
        .await?;

    if LeaderboardSetup::new(
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
