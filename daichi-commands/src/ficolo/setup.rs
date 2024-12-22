use super::*;
use daichi_handlers::on_error_user;
use daichi_models::mongo_crud::MongoCrud;
use daichi_utils::button_selects::{bool_select, role_select};
use poise::command;
use serenity::CreateMessage;

#[command(
    slash_command,
    guild_only,
    on_error = "on_error_user",
    check = "check_no_guild",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn setup(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    if !bool_select(
        ctx,
        "Are you sure you want to manage the ficolo messages in this Channel?\n",
    )
    .await?
    {
        return Ok(());
    }

    let moderator_role = role_select(ctx, "What role should be able to edit and delete?").await?;

    FicoloSetup::new(guild_id, ctx.channel_id(), moderator_role)
        .insert()
        .await?;

    ctx.channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::default().content(
                "This Channel is used to manage this servers questionset.\n".to_string() + 
                "If you want an overview on what questions do and what kinds exist then use:\n" +
                "`/ficolo overview`",
            ),
        )
        .await?;

    Ok(())
}
