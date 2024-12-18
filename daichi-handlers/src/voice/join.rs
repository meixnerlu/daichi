use daichi::*;
use serenity::Mentionable;
use songbird::CoreEvent;

use super::VoiceHandler;

pub async fn handle_join(ctx: &Context<'_>, channel_id: serenity::ChannelId) -> Result<()> {
    let guild_id = ctx.guild_id().expect("the command to be guild only");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    {
        let handler_lock = manager.get_or_insert(guild_id);
        let mut handler = handler_lock.lock().await;

        let event_reciever = VoiceHandler::new(guild_id);

        handler.add_global_event(
            songbird::Event::Core(CoreEvent::VoiceTick),
            event_reciever.clone(),
        );
    }

    if let Ok(_handler_lock) = manager.join(guild_id, channel_id).await {
        ctx.reply(format!("Joined {}", channel_id.mention()))
            .await?;
    } else {
        let _ = manager.remove(guild_id).await;
        ctx.reply(format!("Failed to join {}", channel_id.mention()))
            .await?;
    }

    Ok(())
}
