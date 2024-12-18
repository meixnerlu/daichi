use daichi::*;
use daichi_leaderboard::leaderboards;
use daichi_models::guildsetup::GuildSetup;
use daichi_utils::sync_user_states::sync_user_states;
use role_button::handle_role_toggle;
use voice_event::handle_voice_event;

mod ready;
mod role_button;
mod voice_event;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<()> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => ready::ready(data_about_bot).await?,
        serenity::FullEvent::CacheReady { guilds } => {
            sync_user_states(ctx, guilds.clone()).await?;
            leaderboards(ctx).await?;
        }
        serenity::FullEvent::VoiceStateUpdate { new, .. } => {
            if let Some(guild_id) = new.guild_id {
                if GuildSetup::guild_exists(guild_id).await? {
                    handle_voice_event(new, ctx).await?;
                }
            }
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild.id)
                .await?;
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let Some(button_press) = interaction.clone().message_component() {
                handle_role_toggle(button_press, ctx).await?;
            }
        }
        _ => {}
    }

    Ok(())
}
