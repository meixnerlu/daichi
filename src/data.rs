use std::env;

use crate::prelude::*;
use async_once_cell::OnceCell;
use moka::sync::Cache;
use mongodb::{Client, Database};

const DATABASE: &str = "daichi";
static DATACELL: OnceCell<Data> = OnceCell::new();

type GuildCache = Cache<serenity::GuildId, Option<serenity::RoleId>>;

// I know this is ugly
#[derive(Debug, Clone)]
pub struct DcData();

impl DcData {
    pub async fn data() -> &'static Data {
        Data::global().await
    }
}

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
            db: client.database(DATABASE),
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
