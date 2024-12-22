use daichi::*;
use daichi_handlers::on_error_user;
use daichi_models::guildsetup::GuildSetup;
use poise::command;

mod change_afk;
mod delete;
mod replace;
mod role_button;
mod setup;

/// use `/help leaderboard`
///
/// When set up it will track when users join and leave voicechannels
/// This is used to create a leaderboard that displays the time of 10 users
/// The users are the once with the most time spend in the channels
#[command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands(
        "delete::delete",
        "role_button::role_button",
        "setup::setup",
        "replace::replace",
        "change_afk::change_afk"
    ),
    subcommand_required
)]
pub async fn leaderboard(_: Context<'_>) -> Result<()> {
    Ok(())
}

async fn check_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    GuildSetup::guild_exists(guild_id).await
}

async fn check_no_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    GuildSetup::guild_exists(guild_id).await.map(|guild| !guild)
}
