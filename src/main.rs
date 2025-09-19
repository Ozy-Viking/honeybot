use discord_honey_bot::prelude::*;
use miette::IntoDiagnostic;
use serenity::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().into_diagnostic()?;
    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MODERATION;

    let honey_pot_bot = HoneyPotBot::new(None, None)?;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&discord_token, intents)
        .event_handler(honey_pot_bot)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
    Ok(())
}
