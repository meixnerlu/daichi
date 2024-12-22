use daichi::*;
use daichi_leaderboard::leaderboards;
use daichi_models::{
    leaderboardsetup::LeaderboardSetup, mongo_crud::MongoCrud, role_toggle::RoleToggle,
    user_dc_event::UserDcEvent,
};
use daichi_utils::sync_user_states::sync_user_states;
use role_button::handle_role_toggle;
use voice_event::handle_voice_event;

mod ready;
mod role_button;
mod voice_event;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, DcData, Error>,
    _data: &DcData,
) -> Result<()> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => ready::ready(ctx, data_about_bot).await?,
        serenity::FullEvent::CacheReady { guilds } => {
            sync_user_states(ctx, guilds.clone()).await?;
            leaderboards(ctx).await?;
        }
        serenity::FullEvent::VoiceStateUpdate { new, .. } => {
            if let Some(guild_id) = new.guild_id {
                if LeaderboardSetup::guild_exists(guild_id).await? {
                    handle_voice_event(new, ctx).await?;
                }
            }
        }
        serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            UserDcEvent::delete(
            doc! {"metadata.guild_id": guild_id.to_string(), "metadata.user_id": user.id.to_string()},
            ).await?;
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild.id)
                .await?;
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let Some(button_press) = interaction.clone().message_component() {
                if let Ok(role_toggle) = RoleToggle::from_json(&button_press.data.custom_id) {
                    handle_role_toggle(role_toggle, button_press, ctx).await?;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
