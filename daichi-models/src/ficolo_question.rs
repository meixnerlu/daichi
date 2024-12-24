use std::{fmt::Display, str::FromStr};

use daichi::*;
use serde::{Deserialize, Serialize};

use crate::mongo_crud::MongoCrud;

#[derive(Debug, Serialize, Deserialize)]
pub struct FicoloQuestion {
    guild_id: serenity::GuildId,
    message_id: serenity::MessageId,
    question_text: String,
    rounds: Option<Rounds>,
    status: QuestionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rounds {
    Fixed { rounds: i8 },
    Range { from: i8, to: i8 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuestionStatus {
    Active,
    Deleted,
}

impl FicoloQuestion {
    pub fn new(
        guild_id: impl Into<serenity::GuildId>,
        message_id: impl Into<serenity::MessageId>,
        question_text: impl Into<String>,
        rounds: Option<Rounds>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            message_id: message_id.into(),
            question_text: question_text.into(),
            rounds,
            status: QuestionStatus::Active,
        }
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
            .options(mongodb::options::IndexOptions::builder().build())
            .build();

        let _ = Self::get_collection().await.create_index(index).await;
        Ok(())
    }
}

impl MongoCrud for FicoloQuestion {
    const COLLECTION: &'static str = "ficolo_question";
}

impl FromStr for Rounds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        match parts.as_slice() {
            [single] => {
                let rounds: i8 = single.parse().map_err(Error::from_any)?;
                if (1..=9).contains(&rounds) {
                    Ok(Rounds::Fixed { rounds })
                } else {
                    Err(Error::from("Bad argument"))
                }
            }
            [from, to] => {
                let from: i8 = from.parse().map_err(Error::from_any)?;
                let to: i8 = to.parse().map_err(Error::from_any)?;
                if (1..=9).contains(&from) && (1..=9).contains(&to) && from < to {
                    Ok(Rounds::Range { from, to })
                } else {
                    Err(Error::from("Bad argument"))
                }
            }
            _ => Err(Error::from("Bad argument")),
        }
    }
}

impl Display for Rounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rounds::Fixed { rounds } => write!(f, "{}", rounds),
            Rounds::Range { from, to } => write!(f, "{}-{}", from, to),
        }
    }
}
