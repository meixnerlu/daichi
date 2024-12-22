use daichi::*;
use daichi_models::{mongo_crud::MongoCrud, role_toggle::RoleToggle, user_dc_event::UserDcEvent};
use serenity::CacheHttp;

pub async fn handle_role_toggle(
    role_toggle: RoleToggle,
    interaction: serenity::ComponentInteraction,
    ctx: &serenity::Context,
) -> Result<()> {
    let guild_id = interaction.guild_id.unwrap();

    match interaction
        .user
        .has_role(ctx.http(), guild_id, role_toggle.role_id)
        .await
        .unwrap()
    {
        true => {
            interaction
                .member
                .as_ref()
                .unwrap()
                .remove_role(ctx.http(), role_toggle.role_id)
                .await?;
            interaction
                .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                .await?;
            UserDcEvent::delete(
                doc! {"metadata.guild_id": guild_id.to_string(), "metadata.user_id": interaction.user.id.to_string()},
            ).await?;
        }
        false => {
            interaction
                .member
                .as_ref()
                .unwrap()
                .add_role(ctx.http(), role_toggle.role_id)
                .await?;
            interaction
                .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                .await?;
        }
    }
    Ok(())
}
