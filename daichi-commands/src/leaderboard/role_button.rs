use daichi::*;
use daichi_handlers::on_error_user;
use daichi_models::{guildsetup::GuildSetup, mongo_crud::MongoCrud};
use poise::command;
use serenity::Mentionable;

/// Creates a message with a button where people can get the role that is being watched
#[command(
    slash_command,
    guild_only,
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn role_button(
    ctx: Context<'_>,
    #[description = "The text of the button (can be an emote)"] label: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let guild_setup = GuildSetup::get(doc! {"guild_id": guild_id.to_string()})
        .await?
        .unwrap();

    if guild_setup.role_to_watch.is_none() {
        ctx.reply("Your server did not set a role").await?;
        return Ok(());
    }
    let role = guild_setup.role_to_watch.unwrap();

    let button = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(
            "role_toggle-".to_string() + &guild_id.to_string() + "-" + &role.to_string(),
        )
        .label(label),
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

    Ok(())
}
