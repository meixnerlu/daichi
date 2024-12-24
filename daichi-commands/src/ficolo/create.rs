use daichi_handlers::on_error_user;
use daichi_utils::modals::{FicoloTypes, QuestionModal, RoundsModal, StandardModal};

use super::*;

/// Create a new question
#[command(
    slash_command,
    guild_only,
    check = "is_mod",
    on_error = "on_error_user",
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn create(
    ctx: poise::ApplicationContext<'_, DcData, Error>,
    #[description = "What kind of questions"] kind: FicoloTypes,
) -> Result<()> {
    match kind {
        FicoloTypes::Standard => StandardModal::handle_modal(ctx).await?,
        FicoloTypes::Rounds => RoundsModal::handle_modal(ctx).await?,
    };

    Ok(())
}
