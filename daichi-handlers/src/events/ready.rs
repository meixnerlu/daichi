use daichi::*;
use serenity::ActivityData;

pub async fn ready(ctx: &serenity::Context, data_about_bot: &serenity::Ready) -> Result<()> {
    println!("Bot running as {}", data_about_bot.user.name);
    ctx.set_activity(Some(ActivityData::playing("/help")));
    Ok(())
}
