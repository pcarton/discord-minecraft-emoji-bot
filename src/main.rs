use std::env;

mod secret_file_objects;
mod commands;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse,CreateInteractionResponseMessage,EditInteractionResponse};
use serenity::model::application::{Command,Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            command
                .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::default()))
                .await
                .expect("Expected Loading message to be sent");

                let content = match command.data.name.as_str() {
                    "minecraftemote" => commands::minecraftemote::run(&ctx, &command).await,
                    _ => "Not implemented :(".to_string()
                };


            command
                .edit_response(&ctx.http, EditInteractionResponse::new().content(content))
                    .await
                    .expect("Expected message to be sent");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let filepath = env::var("SECRETS_FILE_PATH").expect("SECRETS_FILE_PATH must be set");

        let file = std::fs::File::open(filepath).expect("Could not open file.");
        let secret_file_contents: secret_file_objects::SecretFile = serde_yaml::from_reader(file).expect("Could not read values.");

        let guild_id_from_file = secret_file_contents.secrets.discord_guild_id;

        let guild_id = GuildId::new(
            guild_id_from_file.parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id.set_commands(&ctx.http, vec![
            commands::minecraftemote::register()
        ])
        .await;

        let _global_command = Command::create_global_command(&ctx.http, commands::minecraftemote::register())
        .await;
    }
}

#[tokio::main]
async fn main() {
    let filepath = env::var("SECRETS_FILE_PATH").expect("SECRETS_FILE_PATH must be set");

    let file = std::fs::File::open(filepath).expect("Could not open file.");
    let secret_file_contents: secret_file_objects::SecretFile = serde_yaml::from_reader(file).expect("Could not read values.");

    // Configure the client with your Discord bot token in the environment.
    let token = secret_file_contents.secrets.discord_api_token;

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

