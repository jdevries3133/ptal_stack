use anyhow::Result;
use reqwest::get;
/// Interface for https://dog.ceo/dog-api/
use serde::Deserialize;

#[derive(Deserialize)]
struct Dog {
    message: String,
}

pub async fn get_photo() -> Result<String> {
    let dog: Dog = get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json()
        .await?;

    Ok(dog.message)
}
