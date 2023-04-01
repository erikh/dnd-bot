use lazy_static::lazy_static;
use regex::Regex;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

lazy_static! {
    static ref DICE_REGEX: Regex =
        Regex::new(r"\s*(([1-9][0-9]*)d)?([1-9][0-9]*)(([+-][1-9][0-9]*))?").unwrap();
}

fn convert_capture(captures: &regex::Captures, index: usize, default: i32) -> i32 {
    match captures.get(index) {
        Some(x) => x.as_str().parse::<i32>().unwrap_or(default),
        None => default,
    }
}

fn roll(text: String) -> String {
    let captures = match DICE_REGEX.captures(&text) {
        Some(c) => c,
        None => {
            return "Please provide a set of dice (e.g., 2d6 or 1d8+1)".to_string();
        }
    };

    let num_dice = convert_capture(&captures, 2, 1);
    let die_size = convert_capture(&captures, 3, 10);
    let offset = convert_capture(&captures, 5, 0);

    let mut dice: Vec<i32> = Vec::new();
    let mut sum: i128 = 0;
    let mut advantage: i32 = 0;
    let mut disadvantage: i32 = 0;

    for _ in 0..num_dice {
        let mut result: i32 = rand::random();
        result = (result.abs() % die_size) + 1; // dice start at 1
        dice.push(result);
        sum += result as i128;
        if result > advantage {
            advantage = result;
        }

        if result < disadvantage || disadvantage == 0 {
            disadvantage = result;
        }
    }

    if dice.len() > 1 {
        format!(
            "sum: {} | advantage: {} | disadvantage: {} | dice: {:?}",
            sum + offset as i128,
            advantage + offset,
            disadvantage + offset,
            dice
        )
    } else {
        format!("sum: {} | dice: {:?}", sum + offset as i128, dice)
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!roll") {
            if let Err(why) = msg.channel_id.say(&ctx.http, roll(msg.content)).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(
        token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
