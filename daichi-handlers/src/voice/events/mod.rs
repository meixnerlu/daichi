use daichi::serenity::{async_trait, GuildId};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use voice_tick::handle_voice_tick;

use super::cache::VoiceCache;

mod voice_tick;

#[derive(Debug, Clone)]
pub struct VoiceHandler {
    inner: Arc<Mutex<InnerVoiceHandler>>,
}

impl VoiceHandler {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerVoiceHandler::new(guild_id))),
        }
    }
}

impl Deref for VoiceHandler {
    type Target = Mutex<InnerVoiceHandler>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone, Default)]
pub struct InnerVoiceHandler {
    guild_id: GuildId,
    buffer_state: u8,
    buffer: Vec<Vec<i16>>,
}

impl InnerVoiceHandler {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            ..Default::default()
        }
    }

    pub fn handle_new_tick(&mut self, tick: Vec<i16>) {
        self.buffer.push(tick);
        if self.buffer_state < 5 {
            self.buffer_state += 1;
        } else {
            self.buffer_state = 0;
            VoiceCache::add(self.guild_id, self.buffer.concat());
            self.buffer.clear();
        }
    }
}

#[async_trait]
impl songbird::EventHandler for VoiceHandler {
    async fn act(&self, ctx: &songbird::EventContext<'_>) -> Option<songbird::Event> {
        #[allow(clippy::single_match)]
        match ctx {
            songbird::EventContext::VoiceTick(tick) => {
                let _ = handle_voice_tick(self, tick).await;
            }
            _ => {}
        }
        None
    }
}
