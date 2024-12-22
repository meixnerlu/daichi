use std::env;

use crate::prelude::*;
use async_once_cell::OnceCell;
use moka::future::Cache;
use mongodb::{Client, Database};

const DATABASE: &str = "daichi";
static DATACELL: OnceCell<Data> = OnceCell::new();

pub type GuildCache = Cache<serenity::GuildId, GuildCacheData>;
pub type GuildCacheData = (Option<serenity::RoleId>, Option<serenity::ChannelId>);

// I want to use OnceCell not the build in data in poise so I dont have to pass it to data
// functions
#[derive(Debug, Clone)]
pub struct DcData();

#[derive(Debug, Clone)]
pub struct Data {
    db: Database,
    guild_cache: GuildCache,
}

impl Data {
    async fn new() -> Self {
        let uri = env::var("MONGODB").expect("MONGODB to be present");
        let client = Client::with_uri_str(uri)
            .await
            .expect("Cannot connect to database");
        Self {
            db: client.database(&env::var("DATABASE").unwrap_or(DATABASE.to_string())),
            guild_cache: GuildCache::new(10_000),
        }
    }

    pub async fn global() -> &'static Self {
        DATACELL.get_or_init(Self::new()).await
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn guild_cache(&self) -> &GuildCache {
        &self.guild_cache
    }
}
