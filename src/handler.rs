use cmd_lib::run_fun;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const SERVER_ERROR: &str = "`Error: Cannot fetch server status!`";
const SERVER_OFFLINE: &str = "`Server offline`";
const SERVER_ONLINE: &str = "`Server online`";

const TTT_COMMAND: &str = "`connect h2879589.stratoserver.net:27015; password <password>`";
const PH_COMMAND: &str = "`connect h2879589.stratoserver.net:27115; password <password>`";
const SL_COMMAND: &str = "`connect h2879589.stratoserver.net:27215; password <password>`";
const MC_SERVER: &str = "`h2879589.stratoserver.net`";
const HELP: &str = "`!ttt` -> TTT server infos.\n\
                    `!ph` -> Prophunt server infos.\n\
                    `!sl` -> Slasher server infos.\n\
                    `!mc` -> Minecraft server infos.\n";

pub struct CommandHandler;

impl EventHandler for CommandHandler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ttt" {
            if let Err(why) = msg.channel_id.say(&ctx.http, create_message(0)) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!ph" {
            if let Err(why) = msg.channel_id.say(&ctx.http, create_message(1)) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!sl" {
            if let Err(why) = msg.channel_id.say(&ctx.http, create_message(2)) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!mc" {
            if let Err(why) = msg.channel_id.say(&ctx.http, create_message(3)) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "!help" {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn create_message(id: i8) -> String {
    return if id == 0 {
        let status = run_fun!("/home/gmodserver/gmodserver details | grep -A 9 \"Server name:\" | sed -r \"s/\\x1B\\[([0-9]{{1,3}}(;[0-9]{{1,2}})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => check_message(status),
            Err(_status) => String::from(SERVER_ERROR)
        };
        String::from(format!("`{}`\n{}", status, TTT_COMMAND))
    } else if id == 1 {
        let status = run_fun!("/home/gmodserver/gmodserver-2 details | grep -A 9 \"Server name:\" | sed -r \"s/\\x1B\\[([0-9]{{1,3}}(;[0-9]{{1,2}})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => check_message(status),
            Err(_status) => String::from(SERVER_ERROR)
        };
        String::from(format!("`{}`\n{}", status, PH_COMMAND))
    } else if id == 2 {
        let status = run_fun!("/home/gmodserver/gmodserver-3 details | grep -A 9 \"Server name:\" | sed -r \"s/\\x1B\\[([0-9]{{1,3}}(;[0-9]{{1,2}})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => check_message(status),
            Err(_status) => String::from(SERVER_ERROR)
        };
        String::from(format!("{}\n{}", status, SL_COMMAND))
    } else {
        let status = run_fun!("/usr/bin/lsof -i -P -n | grep 25565");
        let status = match status {
            Ok(status) => check_mc_status(status),
            Err(_status) => String::from(SERVER_ERROR)
        };
        String::from(format!("{}\n{}", status, MC_SERVER))
    };
}

fn check_message(msg: String) -> String {
    println!("{}", msg);
    return if msg.is_empty() {
        String::from(SERVER_OFFLINE)
    } else {
        msg
    }
}

fn check_mc_status(msg: String) -> String {
    println!("{}", msg);
    return if msg.is_empty() {
        String::from(SERVER_OFFLINE)
    } else {
        String::from(SERVER_ONLINE)
    }
}
