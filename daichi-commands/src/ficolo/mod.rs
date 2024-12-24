use daichi::*;
use daichi_models::ficolosetup::FicoloSetup;
use poise::command;

mod create;
mod overview;
mod setup;

/// use ´/help ficolo´
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("setup::setup", "overview::overview", "create::create"),
    subcommand_required
)]
pub async fn ficolo(_: Context<'_>) -> Result<()> {
    Ok(())
}

async fn is_mod(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();

    match FicoloSetup::get_data(guild_id).await {
        Ok(data) => Ok(ctx.author().has_role(ctx.http(), guild_id, data.0).await?),
        Err(_) => Ok(false),
    }
}

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
