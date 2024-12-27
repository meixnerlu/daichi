use super::*;
use daichi_models::leaderboardsetup::LeaderboardSetup;
use daichi_utils::{button_selects::channel_select, sync_user_states::sync_user_states};

/// Changes the afk channel of the server
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    ephemeral
)]
pub async fn change_afk(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    let afk_channel = channel_select(ctx).await?;

    LeaderboardSetup::change_afk_channel(guild_id, afk_channel).await?;

    let msg = ctx.reply("Updating now...").await?;

    sync_user_states(ctx.serenity_context(), vec![guild_id]).await?;

    msg.delete(ctx).await?;

    Ok(())
}
