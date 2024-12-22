use daichi::*;
use daichi_models::guildsetup::GuildSetup;

pub async fn check_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    GuildSetup::guild_exists(guild_id).await
}

pub async fn check_no_guild(ctx: Context<'_>) -> Result<bool> {
    let guild_id = ctx.guild_id().unwrap();
    GuildSetup::guild_exists(guild_id).await.map(|guild| !guild)
}
