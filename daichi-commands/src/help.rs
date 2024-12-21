use daichi::*;
use poise::command;

/// shows the help texts
///
/// use `/help <command>` to get detailed help
#[command(slash_command, guild_only, ephemeral)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<()> {
    let config = poise::builtins::HelpConfiguration {
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
