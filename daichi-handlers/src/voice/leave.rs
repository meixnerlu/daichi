use daichi::*;

use super::VoiceCache;

pub async fn handle_leave(ctx: &Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().expect("the command to be guild only");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    match manager.get(guild_id).is_some() {
        true => {
            if manager.remove(guild_id).await.is_err() {
                ctx.reply("Error leaving the Voicechannel").await?;
            }
            VoiceCache::clear(&guild_id);
            ctx.reply("Bye bye 👋").await?;
        }
        false => {
            ctx.reply("Not in a Voicechannel").await?;
        }
    }

    Ok(())
}
