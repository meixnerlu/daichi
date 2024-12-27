use super::*;

/// Sends an overview for the ficolo questions
#[command(
    slash_command,
    guild_only,
    check = "check_guild",
    on_error = "on_error_user",
    ephemeral
)]
pub async fn overview(ctx: Context<'_>) -> Result<()> {
    let text = "To create a new question use `/ficolo create`\n".to_string() +
        "In questions you can do `[1-3]` to specify a range of numbers.\n" +
        "You can also do `[p1]` - `[p4]` which will be replaced by random players.\n" +
        "In the same question `[p1]` will always be the same person.\n" +
        "If you specify a rounds question the players will be notified when it ends.\n" +
        "A rounds question can either be a fixed number from 1-9 or a range like `3-5`\n" +
        "You can edit a question by pressing the pen button.\n" +
        "If you delete a question with the button it can later be restored by an admin\n" +
        "An admin can use `/ficolo clean` to delete all messages not from the bot in that channel";

    ctx.reply(text).await?;
    Ok(())
}
