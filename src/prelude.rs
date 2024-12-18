pub type Result<T> = std::result::Result<T, crate::Error>;
pub use poise::serenity_prelude::{self as serenity};
pub type Context<'a> = poise::Context<'a, DcData, crate::Error>;

pub use crate::data::*;
