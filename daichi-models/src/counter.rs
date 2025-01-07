use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};
use serenity::UserId;

use daichi::*;

use crate::mongo_crud::MongoCrud;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Counter {
    guild_id: serenity::GuildId,
    message_id: serenity::MessageId,
    text: String,
    log: Vec<UserId>,
    current: i32,
}

impl Counter {
    pub fn new(
        guild_id: impl Into<serenity::GuildId>,
        message_id: impl Into<serenity::MessageId>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            message_id: message_id.into(),
            text: text.into(),
            log: vec![],
            current: 0,
        }
    }

    pub async fn increment(
        user_id: impl Into<serenity::UserId>,
        guild_id: impl Into<serenity::GuildId>,
        message_id: impl Into<serenity::MessageId>,
    ) -> Result<(i32, String)> {
        let user_id = user_id.into();
        let guild_id = guild_id.into();
        let message_id = message_id.into();

        let mut options = FindOneAndUpdateOptions::default();
        options.return_document = Some(ReturnDocument::After);

        let counter = Self::get_collection()
            .await
            .find_one_and_update(
                doc! {"guild_id": guild_id.to_string(), "message_id": message_id.to_string()},
                doc! {"$inc": {"current": 1}, "$push": {"log": user_id.to_string()}},
            )
            .with_options(Some(options))
            .await?
            .ok_or("not found")?;

        Ok((counter.current, counter.text))
    }
}

impl MongoCrud for Counter {
    const COLLECTION: &'static str = "counter";
}
