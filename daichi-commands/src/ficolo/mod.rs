use daichi::*;
use daichi_handlers::on_error_user;
use daichi_models::ficolosetup::FicoloSetup;
use poise::command;

mod clean;
mod create;
mod overview;
mod setup;

/// use ´/help ficolo´
///
/// Ficolo is a drinking game I wrote an app for my friends but writing apps for IOS costs money
/// So its in this discord bot you can just ignore it if you dont wanna use it
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("setup::setup", "overview::overview", "create::create", "clean::clean"),
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
