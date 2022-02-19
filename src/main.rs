
mod constants;

use std::{
  env,
};

use constants::*;

use serenity::{
	async_trait,
  model::{channel::Message, gateway::Ready},
	prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == LIVE_COMMAND {
      if let Err(why) = msg.channel_id.say(&ctx.http, LIVE_OFF_NELSON).await {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN")
  .expect("Expected a token in the environment");

  let mut client = Client::builder(&token)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}