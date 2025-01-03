use daichi::*;
use daichi_models::{
    leaderboardsetup::LeaderboardSetup,
    mongo_crud::MongoCrud,
    user_dc_event::{UserDcEvent, UserEventType},
};
use serenity::CacheHttp;

pub async fn sync_user_states(
    ctx: &serenity::Context,
    cached_guilds: Vec<serenity::GuildId>,
) -> Result<()> {
    let guilds: Vec<LeaderboardSetup> = LeaderboardSetup::get_guilds()
        .await?
        .iter()
        .filter(|&guild| cached_guilds.contains(&guild.guild_id))
        .cloned()
        .collect();

    for guild in guilds {
        let db_active = UserDcEvent::active_users(guild.guild_id).await?;
        let dc_active = get_dc_active_users(ctx, &guild.guild_id, guild.afk_channel).await?;

        for db_active_user in db_active.clone() {
            if !dc_active.contains(&db_active_user) {
                UserDcEvent::new(guild.guild_id, db_active_user, UserEventType::Left)
                    .insert()
                    .await?;
            }
        }

        for dc_active_user in dc_active {
            if !db_active.contains(&dc_active_user) {
                let user = dc_active_user.to_user(ctx.http()).await?;
                if user.bot {
                    continue;
                }
                match guild.role_to_watch {
                    Some(role) => {
                        if user.has_role(ctx.http(), &guild.guild_id, role).await? {
                            UserDcEvent::new(guild.guild_id, dc_active_user, UserEventType::Joined)
                                .insert()
                                .await?;
                        }
                    }
                    None => {
                        UserDcEvent::new(guild.guild_id, dc_active_user, UserEventType::Joined)
                            .insert()
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn get_dc_active_users(
    ctx: &serenity::Context,
    guild_id: &serenity::GuildId,
    afk_channel: Option<serenity::ChannelId>,
) -> Result<Vec<serenity::UserId>> {
    let out = vec![];

    if let Some(cached_guild) = guild_id.to_guild_cached(ctx.cache().unwrap()) {
        return Ok(cached_guild
            .voice_states
            .iter()
            .filter(|(_, state)| match afk_channel {
                Some(afk_channel) => state
                    .channel_id
                    .is_some_and(|channel_id| channel_id != afk_channel),
                None => state.channel_id.is_some(),
            })
            .map(|(user, _)| *user)
            .collect());
    };

    Ok(out)
}
