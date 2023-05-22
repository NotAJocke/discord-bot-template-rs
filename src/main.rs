use dotenv;
use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use std::env;

mod commands;
mod events;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::ready_event(ctx, ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction_create::interaction_create_event(ctx, interaction).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("CLIENT_TOKEN").expect("Expected 'CLIENT_TOKEN' env variable.");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error on startup: {}", why);
    }
}
