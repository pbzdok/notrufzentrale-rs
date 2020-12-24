use std::env;

use rand::seq::SliceRandom;
use rand::thread_rng;
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
use serenity::utils::Colour;

use crate::parse::parse_dice_str;
use crate::roll::Rolls;

mod parse;
mod roll;

#[derive(Serialize, Deserialize)]
struct Insult {
    insult: String
}

#[derive(Serialize, Deserialize)]
struct Quote {
    quote: String
}

const ANSWERS: [&str; 19] = ["As I see it, yes", "Yes", "No", "Very likely", "Not even close", "Maybe", "Very unlikely", "Janni's mom told me yes", "Janni's mom told me no", "Ask again later", "Better not tell you now", "Concentrate and ask again", "Don't count on it", " It is certain", "My sources say no", "Outlook good", "You may rely on it", "Very Doubtful", "Without a doubt"];

#[group]
#[commands(help, kanye, front, mm, roll, roll_crit)]
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
            e.field("`#kanye`", "Weisheiten von Kanye", false);
            e.field("`#front`", "Lass dich vom Bot fronten", false);
            e.field("`#mm <deine Frage>`", "Frag die magische Miesmuschel", false);
            e.field("`#roll <Würfel>`", "Würfel nach Standard Notation, z.B. `#roll 2d10`", false);
            e.field("`#roll_crit <Würfel>`", "Würfel nach Standard Notation mit Crit, z.B. `#roll_crit 1d4`", false);
            e
        });
        m
    }).await?;
    Ok(())
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
async fn front(ctx: &Context, msg: &Message) -> CommandResult {
    let body = reqwest::get("https://evilinsult.com/generate_insult.php?lang=en&type=json")
        .await?
        .text()
        .await?;

    let i: Insult = serde_json::from_str(&body)?;

    msg.reply(ctx, String::from(&i.insult)).await?;
    Ok(())
}

#[command]
async fn mm(ctx: &Context, msg: &Message) -> CommandResult {
    let answer = ANSWERS.choose(&mut thread_rng());
    msg.reply(ctx, answer.unwrap_or(&"wut")).await?;
    Ok(())
}

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    let input = &msg.content;
    let dice: Vec<&str> = input.split_whitespace().collect();
    let cmd = parse_dice_str(dice[1]);
    match cmd {
        Ok(v) => {
            let rolls = roll::roll_normal(&v);
            let resp = assemble_dice_response(&rolls);
            msg.reply(ctx, resp).await?;
            Ok(())
        }
        Err(e) => {
            msg.reply(ctx, format!("Wrong input you fool! {}", String::from(e))).await?;
            Ok(())
        }
    }
}

#[command]
async fn roll_crit(ctx: &Context, msg: &Message) -> CommandResult {
    let input = &msg.content;
    let dice: Vec<&str> = input.split_whitespace().collect();
    let cmd = parse_dice_str(dice[1]);
    match cmd {
        Ok(v) => {
            let rolls = roll::roll_crit(&v);
            let resp = assemble_dice_response(&rolls);
            msg.reply(ctx, resp).await?;
            Ok(())
        }
        Err(e) => {
            msg.reply(ctx, format!("Wrong input you fool! {}", String::from(e))).await?;
            Ok(())
        }
    }
}

fn assemble_dice_response(rolls: &Rolls) -> String {
    let roll_str: String = rolls
        .0
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" + ");
    let sum_str = rolls.0.iter().sum::<usize>().to_string();
    [roll_str, sum_str].join(" = ")
}