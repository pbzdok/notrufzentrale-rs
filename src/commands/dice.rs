use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::utils::{
    parse::*,
    roll::*,
};

#[command]
pub async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    let input = &msg.content;
    let dice: Vec<&str> = input.split_whitespace().collect();
    let cmd = parse_dice_str(dice[1]);
    match cmd {
        Ok(v) => {
            let rolls = roll_normal(&v);
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
pub async fn roll_crit(ctx: &Context, msg: &Message) -> CommandResult {
    let input = &msg.content;
    let dice: Vec<&str> = input.split_whitespace().collect();
    let cmd = parse_dice_str(dice[1]);
    match cmd {
        Ok(v) => {
            let rolls = roll_critical(&v);
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
