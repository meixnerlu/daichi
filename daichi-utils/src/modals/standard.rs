use bson::doc;
use daichi::serenity::{
    ButtonStyle, CacheHttp, CreateActionRow, CreateButton, CreateMessage, EditMessage,
};
use daichi_models::{
    ficolo_question::FicoloQuestion, ficolosetup::FicoloSetup, mongo_crud::MongoCrud,
};

use super::{CtxWrapper, QuestionModal};

#[derive(Debug, poise::Modal)]
#[name = "Standard question"]
pub struct StandardModal {
    #[name = "Question"]
    #[placeholder = "[p1] has to drink [2-4] times."]
    #[paragraph]
    text: String,
}

impl StandardModal {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl QuestionModal for StandardModal {
    fn to_discord_message(&self) -> String {
        "**Standard:** \n```".to_string() + &self.text + "\n```"
    }

    async fn handle_new(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
        let guild_id = ctx.guild_id().unwrap();

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

        FicoloQuestion::new(guild_id, msg.id, &self.text, None)
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
