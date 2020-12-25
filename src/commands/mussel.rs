use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

const ANSWERS: [&str; 19] = ["As I see it, yes", "Yes", "No", "Very likely", "Not even close", "Maybe", "Very unlikely", "Janni's mom told me yes", "Janni's mom told me no", "Ask again later", "Better not tell you now", "Concentrate and ask again", "Don't count on it", " It is certain", "My sources say no", "Outlook good", "You may rely on it", "Very Doubtful", "Without a doubt"];

#[command]
pub async fn mm(ctx: &Context, msg: &Message) -> CommandResult {
    let answer = ANSWERS.choose(&mut thread_rng());
    msg.reply(ctx, answer.unwrap_or(&"wut")).await?;
    Ok(())
}
