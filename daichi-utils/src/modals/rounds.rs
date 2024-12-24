use std::str::FromStr;

use daichi::serenity::{ButtonStyle, CreateActionRow, CreateButton, CreateMessage};
use daichi_models::{
    ficolo_question::{FicoloQuestion, Rounds},
    ficolosetup::FicoloSetup,
    mongo_crud::MongoCrud,
};

use super::QuestionModal;

#[derive(Debug, poise::Modal)]
pub struct RoundsModal {
    rounds: String,
    text: String,
}

impl QuestionModal for RoundsModal {
    async fn handle_quesion(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
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
                    .content("**Rounds: ".to_string() + &rounds.to_string() + "**\n\n" + &self.text)
                    .components(components),
            )
            .await?;

        FicoloQuestion::new(guild_id, msg.id, &self.text, Some(rounds))
            .insert()
            .await?;

        Ok(())
    }
}
