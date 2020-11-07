use std::env;

use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
struct Quote {
    quote: String
}

#[group]
#[commands(help, kanye)]
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
async fn kanye(ctx: &Context, msg: &Message) -> CommandResult {
    let body = reqwest::get("https://api.kanye.rest")
        .await?
        .text()
        .await?;

    let q: Quote = serde_json::from_str(&body)?;

    msg.reply(ctx, q.quote).await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Zentrale hier, wie kann ich helfen?");
            e.description("Nutze einen der folgenden Befehle");
            e.field("`#kanye`", "Weisheiten von Kanye", false);
            e
        });
        m
    }).await?;
    Ok(())
}