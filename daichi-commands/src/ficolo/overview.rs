use super::*;

/// Sends an overview for the ficolo questions
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn overview(ctx: Context<'_>) -> Result<()> {
    ctx.reply("WIP").await?;
    Ok(())
}
