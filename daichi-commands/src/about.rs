use daichi::*;
use daichi_handlers::on_error_user;
use poise::command;

/// shows some info about the bot and author
#[command(slash_command, ephemeral, on_error = "on_error_user")]
pub async fn about(ctx: Context<'_>) -> Result<()> {
    ctx.reply("Made by [Jaeger](https://github.com/meixnerlu/) using [Poise](https://github.com/serenity-rs/poise) and [Songbird](https://github.com/serenity-rs/songbird/)\njaegerwastaken on Discord").await?;
    Ok(())
}
