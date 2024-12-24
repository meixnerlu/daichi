use super::QuestionModal;

#[derive(Debug, poise::Modal)]
pub struct RoundsModal {
    rounds: String,
    text: String,
}

impl QuestionModal for RoundsModal {
    async fn handle_quesion(&self, ctx: daichi::Context<'_>) -> daichi::Result<()> {
        todo!() // TODO: implement
    }
}
