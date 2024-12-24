use std::future::Future;

use daichi::*;
use poise::{ChoiceParameter, Modal};

mod rounds;
mod standard;

pub use rounds::*;
pub use standard::*;

#[derive(Debug, Clone, ChoiceParameter)]
pub enum FicoloTypes {
    #[name = "standard question"]
    Standard,
    #[name = "multi round"]
    Rounds,
}

pub trait QuestionModal: Modal + Send {
    fn handle_quesion(&self, ctx: Context<'_>) -> impl Future<Output = Result<()>>;

    #[allow(async_fn_in_trait)]
    async fn handle_modal(ctx: poise::ApplicationContext<'_, DcData, Error>) -> Result<()> {
        let data = Self::execute(ctx).await?;
        match data {
            Some(data) => {
                let msg = ctx.reply("Creating question...").await?;
                data.handle_quesion(ctx.into()).await?;
                msg.delete(ctx.into()).await?;
            }
            None => {
                ctx.reply("Nothing was send :(").await?;
            }
        };
        Ok(())
    }
}
