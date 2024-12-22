use daichi::*;
use serde::{Deserialize, Serialize};

use crate::mongo_crud::MongoCrud;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FicoloSetup {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub moderator_role: serenity::RoleId,
}

impl FicoloSetup {
    pub fn new(
        guild_id: impl Into<serenity::GuildId>,
        channel_id: impl Into<serenity::ChannelId>,
        moderator_role: impl Into<serenity::RoleId>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            channel_id: channel_id.into(),
            moderator_role: moderator_role.into(),
        }
    }

    pub async fn guild_exists(guild_id: impl Into<serenity::GuildId>) -> Result<bool> {
        let setup = Self::get_data(guild_id).await;

        Ok(setup.is_ok())
    }

    pub async fn get_data(guild_id: impl Into<serenity::GuildId>) -> Result<FicoloCacheData> {
        let guild_id = guild_id.into();
        let cache = Data::global().await.ficolo_cache();

        cache
            .try_get_with(guild_id, async move {
                match Self::get(doc! {"guild_id": guild_id.to_string()}).await {
                    Ok(setup) => setup
                        .ok_or(std::fmt::Error)
                        .map(|setup| (setup.moderator_role, setup.channel_id)),
                    Err(_) => Err(std::fmt::Error),
                }
            })
            .await
            .map_err(Error::from_any)
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

impl MongoCrud for FicoloSetup {
    const COLLECTION: &'static str = "ficolo_setup";
}
