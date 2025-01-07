use daichi::*;
use daichi_models::counter::Counter;
use serenity::{CacheHttp, ComponentInteraction, CreateInteractionResponse, EditMessage};

pub async fn handle_counter(ctx: &serenity::Context, press: ComponentInteraction) -> Result<()> {
    let user_id = press.user.id;
    let guild_id = press.guild_id.ok_or("not in guild")?;
    let message_id = press.message.id;
    let channel_id = press.channel_id;

    press
        .create_response(ctx.http(), CreateInteractionResponse::Acknowledge)
        .await?;

    let (num, text) = Counter::increment(user_id, guild_id, message_id).await?;

    channel_id
        .edit_message(
            ctx.http(),
            message_id,
            EditMessage::new().content(text + "\n" + &num.to_string()),
        )
        .await?;

    Ok(())
}
