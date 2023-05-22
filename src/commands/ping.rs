use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) {
    let guild_name = interaction
        .guild_id
        .unwrap()
        .to_partial_guild(&ctx.http)
        .await
        .unwrap()
        .name;

    let res = interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|res| {
                    res.content(format!("Pong from {}", guild_name))
                        .ephemeral(true)
                })
        })
        .await;

    if let Err(why) = res {
        println!("Error sending interaction response: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Developer command")
        .dm_permission(false)
}
