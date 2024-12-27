use std::{fs::File, thread::sleep, time::Duration};

use super::{convert_to_wav, VoiceCache};
use daichi::*;
use serenity::CreateMessage;

pub async fn handle_play(ctx: &Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().expect("the command to be guild only");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    match manager.get(guild_id) {
        Some(handler_lock) => {
            let mut handler = handler_lock.lock().await;
            match VoiceCache::get(&guild_id) {
                Some(data) => {
                    VoiceCache::clear(&guild_id);
                    let (file, path) = convert_to_wav(data, guild_id)?;
                    let input = songbird::input::File::new(path.clone());
                    let _ = handler.play_input(input.into());
                    drop(handler);
                    handle_save(ctx, guild_id, file, path).await?;
                }
                None => {
                    ctx.reply("No voice data").await?;
                }
            }
        }
        None => {
            ctx.reply("Not in a Voicechannel").await?;
        }
    }

    Ok(())
}

async fn handle_save(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    file: File,
    path: String,
) -> Result<()> {
    tokio::spawn(async move {
        sleep(Duration::from_secs(150));
        let _ = std::fs::remove_file(path);
    });

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(format!("audiosave-{guild_id}-yes"))
            .label("yes")
            .style(serenity::ButtonStyle::Success),
        serenity::CreateButton::new(format!("audiosave-{guild_id}-no"))
            .label("no")
            .style(serenity::ButtonStyle::Danger),
    ])];
    let msg = ctx
        .send(
            poise::CreateReply::default()
                .content("Do you want to save the highlight?")
                .components(buttons),
        )
        .await?;

    if let Some(interaction) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(90))
        .filter(move |interaction| {
            interaction
                .data
                .custom_id
                .starts_with(&format!("audiosave-{guild_id}"))
        })
        .await
    {
        match interaction.data.custom_id.ends_with("-yes") {
            true => {
                interaction
                    .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                    .await?;
                ctx.channel_id()
                    .send_message(
                        ctx.http(),
                        CreateMessage::default()
                            .content("Here is the last minute of audio")
                            .add_file(
                                serenity::CreateAttachment::file(&file.into(), "funny-moment.wav")
                                    .await?,
                            ),
                    )
                    .await?;
            }
            false => {
                interaction
                    .create_response(ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
                    .await?;
            }
        }
    }

    msg.delete(*ctx).await?;
    Ok(())
}
