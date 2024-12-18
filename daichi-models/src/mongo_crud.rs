use daichi::*;
use serde::{de::DeserializeOwned, Serialize};

#[allow(async_fn_in_trait)]
pub trait MongoCrud
where
    Self: Serialize + DeserializeOwned + Send + Sync,
{
    const COLLECTION: &'static str;

    async fn insert(&self) -> Result<()> {
        Self::get_collection()
            .await
            .insert_one(self)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn get(filter: mongodb::bson::Document) -> Result<Option<Self>> {
        Self::get_collection()
            .await
            .find_one(filter)
            .await
            .map_err(Error::from)
    }

    async fn change(
        filter: mongodb::bson::Document,
        change: mongodb::bson::Document,
    ) -> Result<()> {
        Self::get_collection()
            .await
            .update_many(filter, change)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn delete(filter: mongodb::bson::Document) -> Result<()> {
        Self::get_collection()
            .await
            .delete_one(filter)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    async fn get_collection() -> mongodb::Collection<Self> {
        Self::get_database()
            .await
            .collection::<Self>(Self::COLLECTION)
    }

    async fn get_database() -> &'static mongodb::Database {
        Data::global().await.db()
    }
}
