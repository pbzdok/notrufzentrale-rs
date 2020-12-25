use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group,
    },
    StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::utils::Colour;

use commands::{
    dice::*,
    mussel::*,
};

mod commands;

#[group]
#[commands(help, mm, roll, roll_crit)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Zentrale hier, wie kann ich helfen?");
            e.color(Colour::DARK_TEAL);
            e.field("`#mm <deine Frage>`", "Frag die magische Miesmuschel", false);
            e.field("`#roll <Würfel>`", "Würfel nach Standard Notation, z.B. `#roll 2d10`", false);
            e.field("`#roll_crit <Würfel>`", "Würfel nach Standard Notation mit Crit, z.B. `#roll_crit 1d4`", false);
            e
        });
        m
    }).await?;
    Ok(())
}