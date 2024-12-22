use daichi::*;
use daichi_handlers::on_error_user;
use poise::command;

#[command(
    slash_command,
    guild_only,
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn setup(_: Context<'_>) -> Result<()> {
    Ok(())
}
