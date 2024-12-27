use std::error::Error;

use daichi::*;
use poise::FrameworkError;

pub async fn on_error_owner(error: FrameworkError<'_, DcData, daichi::Error>) {
    error
        .ctx()
        .unwrap()
        .reply(format!("{}:\n{:#?}", error, error.source()))
        .await
        .unwrap();
}
pub async fn on_error_user(error: FrameworkError<'_, DcData, daichi::Error>) {
    println!("{error}");

    error
        .ctx()
        .unwrap()
        .reply("Sadly your command resulted in an error :(")
        .await
        .unwrap();
}
