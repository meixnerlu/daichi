use super::*;
use daichi_models::{
    leaderboardsetup::LeaderboardSetup, mongo_crud::MongoCrud, role_toggle::RoleToggle,
};
use serenity::Mentionable;

/// Creates a message with a button where people can get the role that is being watched
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn role_button(
    ctx: Context<'_>,
    #[description = "The text of the button (can be an emote)"] label: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let guild_setup = LeaderboardSetup::get(doc! {"guild_id": guild_id.to_string()})
        .await?
        .unwrap();

    if guild_setup.role_to_watch.is_none() {
        ctx.reply("Your server did not set a role").await?;
        return Ok(());
    }
    let role = guild_setup.role_to_watch.unwrap();

    let button = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(RoleToggle::new(role).to_json()?).label(label),
    ])];
    ctx.channel_id()
        .send_message(
            ctx.http(),
            serenity::CreateMessage::default()
                .content(
                    "Click the button to toggle the ".to_string()
                        + &role.mention().to_string()
                        + " Role\n"
                        + "Removing the role removes all your past data on this server",
                )
                .components(button),
        )
        .await?;
    ctx.reply("Done").await?;
    Ok(())
}
