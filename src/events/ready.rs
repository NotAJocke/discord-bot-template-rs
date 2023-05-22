use std::env;

use serenity::{
    model::prelude::{GuildId, Ready},
    prelude::Context,
};

use crate::commands;

pub async fn ready_event(ctx: Context, ready: Ready) {
    println!("Client ready, connected as {}", ready.user.tag());

    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected 'GUILD_ID' env var")
            .parse()
            .expect("'GUILD_ID' env var format error"),
    );
    let guild_cmds = GuildId::set_application_commands(&guild_id, &ctx.http, |cmds| {
        cmds.create_application_command(|cmd| commands::ping::register(cmd))
    })
    .await;

    if let Err(why) = guild_cmds {
        println!("Failed to register guild commands: {}", why);
    }
}
