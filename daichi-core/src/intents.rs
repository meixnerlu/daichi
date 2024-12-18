use daichi::serenity::GatewayIntents;

pub fn get_intents() -> GatewayIntents {
    GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS
}
