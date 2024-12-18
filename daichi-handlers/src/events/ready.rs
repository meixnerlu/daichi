use daichi::*;

pub async fn ready(data_about_bot: &serenity::Ready) -> Result<()> {
    println!("Bot running as {}", data_about_bot.user.name);
    Ok(())
}
