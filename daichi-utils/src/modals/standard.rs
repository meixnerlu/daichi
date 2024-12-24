use daichi::serenity::{ButtonStyle, CreateActionRow, CreateButton, CreateMessage};
use daichi_models::{
    ficolo_question::FicoloQuestion, ficolosetup::FicoloSetup, mongo_crud::MongoCrud,
};

use super::QuestionModal;

#[derive(Debug, poise::Modal)]
pub struct StandardModal {
    text: String,
}

impl QuestionModal for StandardModal {
    async fn handle_quesion(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
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
                    .content("**Standard:** \n\n".to_string() + &self.text)
                    .components(components),
            )
            .await?;

        FicoloQuestion::new(guild_id, msg.id, &self.text, None)
            .insert()
            .await?;

        Ok(())
    }
}
