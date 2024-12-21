use daichi::*;
use serde::{Deserialize, Serialize};
use serenity::futures::StreamExt;

use crate::mongo_crud::MongoCrud;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSetup {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub role_to_watch: Option<serenity::RoleId>,
    pub leaderboard_message: serenity::MessageId,
}

impl GuildSetup {
    pub fn new(
        guild_id: impl Into<serenity::GuildId>,
        channel_id: impl Into<serenity::ChannelId>,
        role_to_watch: Option<serenity::RoleId>,
        leaderboard_message: impl Into<serenity::MessageId>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            role_to_watch,
            leaderboard_message: leaderboard_message.into(),
        }
    }

    pub async fn remove(guild_id: impl Into<serenity::GuildId>) -> Result<()> {
        let guild_id = guild_id.into();
        let state = Data::global().await;

        Self::delete(doc! {"guild_id": guild_id.to_string()}).await?;

        state.guild_cache().remove(&guild_id).await;

        Ok(())
    }

    pub async fn get_guilds() -> Result<Vec<Self>> {
        let cache = Data::global().await.guild_cache();

        let mut cursor = Self::get_collection().await.find(doc! {}).await?;

        let mut out = vec![];

        while let Some(guild) = cursor.next().await {
            let guild = guild?;

            out.push(guild.clone());
            cache.insert(guild.guild_id, guild.role_to_watch).await;
        }

        Ok(out)
    }

    pub async fn guild_exists(guild_id: impl Into<serenity::GuildId>) -> Result<bool> {
        let setup = Self::get_data(guild_id).await;

        Ok(setup.is_ok())
    }

    pub async fn get_data(
        guild_id: impl Into<serenity::GuildId>,
    ) -> Result<Option<serenity::RoleId>> {
        let guild_id = guild_id.into();
        let cache = Data::global().await.guild_cache();

        cache
            .try_get_with(guild_id, async move {
                match Self::get(doc! {"guild_id": guild_id.to_string()}).await {
                    Ok(setup) => setup
                        .ok_or(std::fmt::Error)
                        .map(|setup| setup.role_to_watch),
                    Err(_) => Err(std::fmt::Error),
                }
            })
            .await
            .map_err(Error::from_any)
    }

    pub async fn change_message_id(
        guild_id: impl Into<serenity::GuildId>,
        channel_id: impl Into<serenity::ChannelId>,
        message_id: impl Into<serenity::MessageId>,
    ) -> Result<()> {
        let guild_id = guild_id.into();
        let channel_id = channel_id.into();
        let message_id = message_id.into();

        Self::change(
            doc! {"guild_id": guild_id.to_string()}, 
            doc! {"$set": {"channel_id": channel_id.to_string(), "leaderboard_message": message_id.to_string()}}
        ).await
    }

    pub async fn setup_collection() -> Result<()> {
        let db = Self::get_database().await;

        let options = mongodb::options::CreateCollectionOptions::default();

        let _ = db
            .create_collection(Self::COLLECTION)
            .with_options(options)
            .await;

        let index = mongodb::IndexModel::builder()
            .keys(doc! {"guild_id": 1})
            .options(
                mongodb::options::IndexOptions::builder()
                    .unique(true)
                    .build(),
            )
            .build();

        let _ = Self::get_collection().await.create_index(index).await;
        Ok(())
    }
}

impl MongoCrud for GuildSetup {
    const COLLECTION: &'static str = "guild_setup";

    async fn insert(&self) -> Result<()> {
        Self::get_collection().await.insert_one(self).await?;

        let cache = Data::global().await.guild_cache();

        cache.insert(self.guild_id, self.role_to_watch).await;

        Ok(())
    }

    async fn change(
        filter: mongodb::bson::Document,
        change: mongodb::bson::Document,
    ) -> Result<()> {
        Self::get_collection()
            .await
            .update_many(filter.clone(), change)
            .await?;

        let cache = Data::global().await.guild_cache();
        let guild_id: serenity::GuildId = filter
            .get_str("guild_id")
            .unwrap()
            .parse::<u64>()
            .unwrap()
            .into();

        cache.remove(&guild_id).await;

        Ok(())
    }
}
