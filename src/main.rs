extern crate serenity;
extern crate kankyo;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    kankyo::load()
        .expect(".env file can not be opened");

    let token = kankyo::key("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
