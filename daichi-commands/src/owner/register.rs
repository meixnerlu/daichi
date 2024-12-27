use daichi::*;
use daichi_handlers::on_error_owner;
use poise::command;

#[command(
    slash_command,
    guild_only,
    ephemeral,
    on_error = "on_error_owner",
    owners_only
)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
