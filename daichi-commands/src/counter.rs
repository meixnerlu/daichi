use daichi::*;
use daichi_models::{counter::Counter, mongo_crud::MongoCrud};
use poise::command;
use serenity::{CreateActionRow, CreateButton, CreateMessage};

/// shows the help texts
///
/// use `/help <command>` to get detailed help
#[command(slash_command, guild_only, ephemeral)]
pub async fn counter(ctx: Context<'_>, text: String, label: Option<String>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    let button = vec![CreateActionRow::Buttons(vec![
        CreateButton::new("counter").label(label.unwrap_or("+1".to_string()))
    ])];

    let message = ctx
        .channel_id()
        .send_message(
            ctx.http(),
            CreateMessage::new()
                .content(text.clone() + "\n0")
                .components(button),
        )
        .await?;

    Counter::new(guild_id, message.id, text).insert().await?;

    Ok(())
}
