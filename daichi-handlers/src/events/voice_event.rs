use daichi::*;
use daichi_models::{
    guildsetup::GuildSetup,
    mongo_crud::MongoCrud,
    user_dc_event::{UserDcEvent, UserEventType},
};
use serenity::CacheHttp;

pub async fn handle_voice_event(new: &serenity::VoiceState, ctx: &serenity::Context) -> Result<()> {
    let guild_id = new.guild_id.unwrap();
    let user_id = new.user_id;
    let role = GuildSetup::get_data(guild_id).await?;
    let user = new.user_id.to_user(ctx.http()).await?;

    if user.bot {
        return Ok(());
    }

    if let Some(role) = role {
        if !user.has_role(ctx.http(), guild_id, role).await? {
            return Ok(());
        }
    };

    match (
        UserDcEvent::user_is_active(user_id, guild_id).await?,
        new.channel_id.is_some(),
    ) {
        (true, false) => {
            UserDcEvent::new(guild_id, user_id, UserEventType::Left)
                .insert()
                .await?;
        }
        (false, true) => {
            UserDcEvent::new(guild_id, user_id, UserEventType::Joined)
                .insert()
                .await?;
        }
        _ => {}
    }

    Ok(())
}
