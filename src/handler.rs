use cmd_lib::run_fun;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const TTT_COMMAND: &str = "`connect h2879589.stratoserver.net:27015; password <password>`";
const PH_COMMAND: &str = "`connect h2879589.stratoserver.net:27115; password <password>`";
const SL_COMMAND: &str = "`connect h2879589.stratoserver.net:27215; password <password>`";
const MC_SERVER: &str = "`bte.mcs.lol`";
const HELP: &str = "`!ttt` -> G-Unit TTT server infos.\n\
                    `!ph` -> G-Unit Prophunt server infos.\n\
                    `!sl` -> G-Unit Slasher server infos.\n\
                    `!mc` -> Get the G-Unit Minecraft server address.\n";

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
            if let Err(why) = msg.channel_id.say(&ctx.http, MC_SERVER) {
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
        let status = run_fun!("/home/gmodserver/gmodserver details | grep -A 9 \"Server name:\" | sed -r \"s/\x1B\\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => status,
            Err(status) => String::from("Error: Cannot fetch server status!")
        };
        String::from(format!("{}\n{}", status, TTT_COMMAND))
    } else if id == 1 {
        let status = run_fun!("/home/gmodserver/gmodserver-2 details | grep -A 9 \"Server name:\" | sed -r \"s/\x1B\\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => status,
            Err(status) => String::from("Error: Cannot fetch server status!")
        };
        String::from(format!("{}\n{}", status, PH_COMMAND))
    } else {
        let status = run_fun!("/home/gmodserver/gmodserver-3 details | grep -A 9 \"Server name:\" | sed -r \"s/\x1B\\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g\"");
        let status = match status {
            Ok(status) => status,
            Err(status) => String::from("Error: Cannot fetch server status!")
        };
        String::from(format!("{}\n{}", status, SL_COMMAND))
    };
}
