use daichi::*;
use daichi_models::{
    ficolo_question::FicoloQuestion, ficolosetup::FicoloSetup, mongo_crud::MongoCrud,
};
use daichi_utils::modals::{QuestionModal, RoundsModal, StandardModal};
use serenity::{CacheHttp, ComponentInteraction, CreateInteractionResponseFollowup};

pub async fn handle_ficolo(
    ctx: &serenity::Context,
    interaction: ComponentInteraction,
) -> Result<()> {
    let custom_id = &interaction.data.custom_id;
    let guild_id = interaction.guild_id.unwrap();

    let (role_id, channel_id) = FicoloSetup::get_data(guild_id).await?;

    if !interaction
        .member
        .as_ref()
        .unwrap()
        .roles
        .contains(&role_id)
    {
        interaction
            .create_followup(
                ctx.http(),
                CreateInteractionResponseFollowup::new()
                    .content("You dont have the moderator role"),
            )
            .await?;
        return Ok(());
    }

    let question = FicoloQuestion::get(doc! {
        "message_id": interaction.message.id.to_string(),
        "guild_id": interaction.guild_id.unwrap().to_string(),
    })
    .await?;

    if let Some(question) = question {
        match (custom_id.contains("edit"), custom_id.contains("delete")) {
            (true, false) => match question.rounds {
                Some(rounds) => {
                    RoundsModal::new(rounds.to_string(), question.question_text)
                        .handle_update(ctx, interaction)
                        .await?;
                }
                None => {
                    StandardModal::new(question.question_text)
                        .handle_update(ctx, interaction)
                        .await?;
                }
            },
            (false, true) => {
                println!("test");
                FicoloQuestion::change(
                    doc! {
                        "guild_id": question.guild_id.to_string(),
                        "message_id": question.message_id.to_string(),
                    },
                    doc! {
                        "$set": {
                            "status": "Deleted"
                        }
                    },
                )
                .await?;

                channel_id
                    .delete_message(ctx.http(), question.message_id)
                    .await?;
            }
            _ => {}
        }
    }

    Ok(())
}
