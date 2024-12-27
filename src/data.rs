use std::env;

use crate::prelude::*;
use async_once_cell::OnceCell;
use moka::future::Cache;
use mongodb::{Client, Database};

const DATABASE: &str = "daichi";
static DATACELL: OnceCell<Data> = OnceCell::new();

pub type LeaderboardCache = Cache<serenity::GuildId, LeaderboardCacheData>;
pub type LeaderboardCacheData = (Option<serenity::RoleId>, Option<serenity::ChannelId>);

pub type FicoloCache = Cache<serenity::GuildId, FicoloCacheData>;
pub type FicoloCacheData = (serenity::RoleId, serenity::ChannelId);

// I want to use OnceCell not the build in data in poise so I dont have to pass it to data
// functions
#[derive(Debug, Clone)]
pub struct DcData();

#[derive(Debug, Clone)]
pub struct Data {
    db: Database,
    leaderboard_cache: LeaderboardCache,
    ficolo_cache: FicoloCache,
}

impl Data {
    async fn new() -> Self {
        let uri = env::var("MONGODB").expect("MONGODB to be present");
        let client = Client::with_uri_str(uri)
            .await
            .expect("Cannot connect to database");
        Self {
            db: client.database(&env::var("DATABASE").unwrap_or(DATABASE.to_string())),
            leaderboard_cache: LeaderboardCache::new(1_000),
            ficolo_cache: FicoloCache::new(1_000),
        }
    }

    pub async fn global() -> &'static Self {
        DATACELL.get_or_init(Self::new()).await
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn ficolo_cache(&self) -> &FicoloCache {
        &self.ficolo_cache
    }

    pub fn leaderboard_cache(&self) -> &LeaderboardCache {
        &self.leaderboard_cache
    }
}
