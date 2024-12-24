use super::QuestionModal;

#[derive(Debug, poise::Modal)]
pub struct StandardModal {
    text: String,
}

impl QuestionModal for StandardModal {
    async fn handle_quesion(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
        todo!() // TODO: implement
    }
}
