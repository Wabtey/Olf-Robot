mod gacha;
mod constants;

use std::env;
use gacha::*;
use constants::*;

use serenity::{
	async_trait,
  model::{channel::Message, gateway::Ready, Embed},
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

    if msg.content == ROLL_COMMAND {

      let embed = Embed::fake(|e| e
        .title("Your Roll")
        .description("Print your roll")
        .field(|f| f
            .name("A field")
            .value("Has some content.")
            .inline(false))
        .image("Always monkey_stamp"));

      let results = rolls(1);
      for i in results {
        let f = [(
          &tokio::fs::File::open(i).await?, i
        )];
        if let Err(why) =
          msg
            .channel_id
            .send_message(&ctx.http, |m| {
              // Reply to the given message
              m.reference_message(&replied_message);
              // Ping the replied user
              m.allowed_mentions(|am| {
                  am.replied_user(true);
                  am
              });
            
              // Attach image
              m.files(f);
              m.await {
                  println!("Error sending message: {:?}", why);
              }
            }
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

// channel.edit_message(http, message_id, |msg| msg.embed(|e| { *e = embed; e } ))?;

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