use daichi::*;
use daichi_handlers::on_error_owner;
use poise::command;

#[command(slash_command, guild_only, ephemeral, on_error = "on_error_owner")]
pub async fn throw_error(ctx: Context<'_>) -> Result<()> {
    ctx.http()
        .ban_user(u64::MAX.into(), u64::MAX.into(), 0, None)
        .await?;
    Ok(())
}
