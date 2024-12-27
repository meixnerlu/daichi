use daichi_utils::modals::{FicoloTypes, QuestionModal, RoundsModal, StandardModal};

use super::*;

/// Create a new question
#[command(
    slash_command,
    guild_only,
    check = "is_mod",
    on_error = "on_error_user",
    ephemeral
)]
pub async fn create(
    ctx: poise::ApplicationContext<'_, DcData, Error>,
    #[description = "What kind of questions"] kind: FicoloTypes,
) -> Result<()> {
    match kind {
        FicoloTypes::Standard => StandardModal::handle_new_modal(ctx).await?,
        FicoloTypes::Rounds => RoundsModal::handle_new_modal(ctx).await?,
    };

    Ok(())
}
