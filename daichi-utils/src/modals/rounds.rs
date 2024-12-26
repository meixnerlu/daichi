use std::str::FromStr;

use daichi::*;
use daichi_models::{
    ficolo_question::{FicoloQuestion, Rounds},
    ficolosetup::FicoloSetup,
    mongo_crud::MongoCrud,
};
use serenity::{ButtonStyle, CacheHttp, CreateActionRow, CreateButton, CreateMessage, EditMessage};

use super::{CtxWrapper, QuestionModal};

#[derive(Debug, poise::Modal)]
#[name = "Rounds question"]
pub struct RoundsModal {
    #[name = "Rounds"]
    #[min_length = 1]
    #[max_length = 3]
    #[placeholder = "x or x-y"]
    rounds: String,
    #[name = "Question"]
    #[placeholder = "For (rounds) rounds (player1) has to drink double"]
    #[paragraph]
    text: String,
}

impl RoundsModal {
    pub fn new(rounds: String, text: String) -> Self {
        Self { rounds, text }
    }
}

impl QuestionModal for RoundsModal {
    fn to_discord_message(&self) -> String {
        "**Rounds: ".to_string() + &self.rounds + "**\n```" + &self.text + "\n```"
    }

    async fn handle_new(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
        let guild_id = ctx.guild_id().unwrap();
        let rounds = Rounds::from_str(&self.rounds)?;

        let (_, channel_id) = FicoloSetup::get_data(guild_id).await?;

        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new("ficolo-edit")
                .style(ButtonStyle::Primary)
                .label("🖊️"),
            CreateButton::new("ficolo-delete")
                .style(ButtonStyle::Danger)
                .label("🗑️"),
        ])];

        let msg = channel_id
            .send_message(
                ctx.http(),
                CreateMessage::default()
                    .content(self.to_discord_message())
                    .components(components),
            )
            .await?;

        FicoloQuestion::new(guild_id, msg.id, &self.text, Some(rounds))
            .insert()
            .await?;

        Ok(())
    }

    async fn handle_update(
        self,
        ctx: &daichi::serenity::Context,
        interaction: daichi::serenity::ComponentInteraction,
    ) -> daichi::Result<()> {
        let guild_id = interaction.guild_id.unwrap();
        let channel_id = interaction.channel_id;
        let message_id = interaction.message.id;

        let ctx: CtxWrapper = ctx.to_owned().into();
        if let Some(data) = poise::execute_modal_on_component_interaction::<Self>(
            ctx.clone(),
            interaction,
            Some(self),
            None,
        )
        .await?
        {
            FicoloQuestion::change(
                doc! {
                    "message_id": message_id.to_string(),
                    "guild_id": guild_id.to_string(),
                },
                doc! {
                    "$set": {
                        "question_text": data.text.clone(),
                        "rounds": Some(bson::to_document(&Rounds::from_str(&data.rounds)?).map_err(Error::from_any)?)
                    }
                },
            )
            .await?;

            channel_id
                .edit_message(
                    ctx.ctx.http(),
                    message_id,
                    EditMessage::new().content(data.to_discord_message()),
                )
                .await?;
        };
        Ok(())
    }
}
