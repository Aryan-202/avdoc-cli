use dialoguer::{Input, Password};

use crate::config::provider_config::ProviderConfig;

pub fn connect_deepseek() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Password::new()
        .with_prompt("Enter your DeepSeek API key")
        .interact()?;

    let model: String = Input::new()
        .with_prompt("Enter the model name")
        .default("deepseek-chat".into()) // Pressing Enter keeps this default
        .interact_text()?;

    tokio::runtime::Runtime::new()?.block_on(async {
        reqwest::Client::new()
            .get("https://api.deepseek.com/models")
            .bearer_auth(&api_key)
            .send()
            .await?
            .error_for_status()?;

        println!("Successfully authenticated! Selected model: {}", model);
        Ok::<_, reqwest::Error>(())
    })?;

    let mut config = ProviderConfig::load()?;
    config.set_api_key("deepseek", api_key);
    config.set_default("deepseek", "deepseek-chat");
    config.set_provider_model("deepseek", "deepseek-chat");
    config.save()?;
    println!("Connected to DeepSeek.");

    Ok(())
}
