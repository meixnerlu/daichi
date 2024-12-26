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
    fn handle_new(&self, ctx: Context<'_>) -> impl Future<Output = Result<()>>;

    fn to_discord_message(&self) -> String;

    fn handle_update(
        self,
        ctx: &serenity::Context,
        interaction: serenity::ComponentInteraction,
    ) -> impl Future<Output = Result<()>>;

    #[allow(async_fn_in_trait)]
    async fn handle_new_modal(ctx: poise::ApplicationContext<'_, DcData, Error>) -> Result<()> {
        let data = Self::execute(ctx).await?;
        match data {
            Some(data) => {
                let msg = ctx.reply("Creating question...").await?;
                data.handle_new(ctx.into()).await?;
                msg.delete(ctx.into()).await?;
            }
            None => {
                ctx.reply("Nothing was send :(").await?;
            }
        };
        Ok(())
    }
}

// HACK:
// So the events only get a serenity context and the execute_on_interaction needs something that
// turns into a serenity context
// this just makes that problem go away
#[derive(Debug, Clone)]
struct CtxWrapper {
    ctx: serenity::Context,
}

impl From<serenity::Context> for CtxWrapper {
    fn from(value: serenity::Context) -> Self {
        Self { ctx: value }
    }
}

impl AsRef<serenity::Context> for CtxWrapper {
    fn as_ref(&self) -> &serenity::Context {
        &self.ctx
    }
}
