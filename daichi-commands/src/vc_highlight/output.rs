use daichi::*;
use daichi_handlers::convert_to_wav;
use daichi_handlers::on_error_user;
use daichi_handlers::VoiceCache;
use poise::{command, CreateReply};

/// Safes the last minute directly as a wav file
#[command(
    slash_command,
    guild_only,
    on_error = "on_error_user",
    guild_cooldown = 10
)]
pub async fn output_wav(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    let waiting_msg = ctx.reply("Waiting for data...").await?;
    match VoiceCache::get(&guild_id) {
        Some(data) => {
            VoiceCache::clear(&guild_id);
            let (file, path) = convert_to_wav(data, guild_id)?;

            waiting_msg.delete(ctx).await?;
            let waiting_msg = ctx.reply("Sending file to discord...").await?;
            ctx.send(
                CreateReply::default()
                    .content("Here is the last minute of audio")
                    .attachment(
                        serenity::CreateAttachment::file(&file.into(), "funny-moment.wav").await?,
                    ),
            )
            .await?;
            waiting_msg.delete(ctx).await?;
            std::fs::remove_file(path).map_err(Error::from_any)?;
        }
        None => {
            waiting_msg.delete(ctx).await?;
            ctx.reply("No audio in your server. Is the bot in a Voicechat?")
                .await?;
        }
    };

    Ok(())
}
