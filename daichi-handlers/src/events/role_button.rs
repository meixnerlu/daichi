use daichi::*;
use daichi_models::{mongo_crud::MongoCrud, user_dc_event::UserDcEvent};
use serenity::CacheHttp;

pub async fn handle_role_toggle(
    button_press: serenity::ComponentInteraction,
    ctx: &serenity::Context,
) -> Result<()> {
    if !button_press.data.custom_id.starts_with("role_toggle-") {
        return Ok(());
    }
    let mut args = button_press.data.custom_id.split("-");
    let _ = args.next().unwrap();
    let guild_id: serenity::GuildId = args.next().unwrap().parse::<u64>().unwrap().into();
    let role_id: serenity::RoleId = args.next().unwrap().parse::<u64>().unwrap().into();

    match button_press
        .user
        .has_role(ctx.http(), guild_id, role_id)
        .await
        .unwrap()
    {
        true => {
            button_press
                .member
                .as_ref()
                .unwrap()
                .remove_role(ctx.http(), role_id)
                .await?;
            button_press
                .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                .await?;
            UserDcEvent::delete(
                doc! {"metadata.guild_id": guild_id.to_string(), "metadata.user_id": button_press.user.id.to_string()},
            ).await?;
        }
        false => {
            button_press
                .member
                .as_ref()
                .unwrap()
                .add_role(ctx.http(), role_id)
                .await?;
            button_press
                .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                .await?;
        }
    }
    Ok(())
}
