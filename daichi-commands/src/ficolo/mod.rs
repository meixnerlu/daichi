use daichi::*;
use daichi_models::ficolosetup::FicoloSetup;
use poise::command;

mod overview;
mod setup;

/// use ´/help ficolo´
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("setup::setup"),
    subcommand_required
)]
pub async fn ficolo(_: Context<'_>) -> Result<()> {
    Ok(())
}

#[allow(dead_code)] // idk why but rust-analyzer thinks its unused
async fn check_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    FicoloSetup::guild_exists(guild_id).await
}
async fn check_no_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    FicoloSetup::guild_exists(guild_id)
        .await
        .map(|guild| !guild)
}
