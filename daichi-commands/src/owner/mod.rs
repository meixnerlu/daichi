use daichi::*;
use poise::command;

mod clear_bot_msg;
mod register;
mod throw_error;

#[command(
    slash_command,
    guild_only,
    owners_only,
    hide_in_help,
    subcommands(
        "register::register",
        "clear_bot_msg::clear_bot_msg",
        "throw_error::throw_error"
    ),
    subcommand_required
)]
pub async fn owner(_: Context<'_>) -> Result<()> {
    Ok(())
}
