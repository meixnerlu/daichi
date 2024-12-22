use super::*;
use daichi_models::{guildsetup::GuildSetup, mongo_crud::MongoCrud, user_dc_event::UserDcEvent};
use daichi_utils::button_selects::bool_select;

/// Deletes all data of all users on your server and stops watching the server
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn delete(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    if bool_select(ctx, "Are you sure you want to remove this server?").await? {
        GuildSetup::remove(guild_id).await?;
        UserDcEvent::delete(doc! {"metadata.guild_id": guild_id.to_string()}).await?;
    }
    Ok(())
}
