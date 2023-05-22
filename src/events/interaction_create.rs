use serenity::{model::prelude::interaction::Interaction, prelude::Context};

use crate::commands;
pub async fn interaction_create_event(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(cmd) = interaction {
        println!("Received interaction {}", cmd.data.name);

        match cmd.data.name.as_str() {
            "ping" => commands::ping::run(ctx, cmd).await,
            _ => println!("Not implemented yet"),
        };
    };
}
