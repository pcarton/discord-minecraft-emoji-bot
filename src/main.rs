use std::env;

mod skins;
mod minecraft_api_objects;
mod secret_file_objects;

use serenity::async_trait;
use serenity::builder::{CreateCommand,CreateCommandOption,CreateInteractionResponse,CreateInteractionResponseMessage,EditInteractionResponse};
use serenity::model::application::{Command,CommandInteraction,CommandOptionType,CommandDataOptionValue,Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

fn check_permissions(command: CommandInteraction) -> bool {
    let user_permissions_valid = command
        .member
        .clone()
        .expect("Expect member to be there")
        .permissions
        .expect("Expect the permissions Object to be there")
        .manage_guild_expressions();

    let bot_permissions_valid = command
        .app_permissions
        .expect("Expect the app_permissions Object to be there")
        .manage_guild_expressions();

    user_permissions_valid && bot_permissions_valid
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            command
                .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::default()))
                .await
                .expect("Expected Loading message to be sent");

                let content = match command.data.name.as_str() {
                    "minecraftemote" => {
                        let options = &command
                            .data
                            .options
                            .get(0)
                            .expect("Expected minecraft_username option")
                            .value;

                        if command.guild_id.is_none() {
                            "This command cannot be executed in a direct message".to_string()
                        // Check if the bot and user have the required permissions
                        } else if !check_permissions(command.clone()) {
                            "I do not have the Manage Emoji and Stickers permission in my role, please have your admin add them".to_string()
                        } else if let CommandDataOptionValue::String(minecraft_username) = options {
                            let local_emote_path = skins::download_face(minecraft_username.clone())
                                .await
                                .expect("Expect Skin Face to Download");

                            let emoji_name = format!("{}Minecraft",minecraft_username);

                            let guild = command.guild_id.expect("Expect GuildID");

                            let emoji_face = GuildId::create_emoji(guild, &ctx, &emoji_name, &local_emote_path)
                                .await
                                .expect("Expected emoji to be created from file");

                            format!("Emoji created as {}", emoji_face)
                        } else {
                            "Issue parsing minecraft_username".to_string()
                        }
                    },
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

        let minecraft_command = CreateCommand::new("minecraftemote")
            .description("Create an Emote for the Server based on a Minecraft User's Skin")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "minecraft_username", "The Minecraft Username of the User you want to make an Emote of")
                    .required(true)
            );

        let _commands = guild_id.set_commands(&ctx.http, vec![minecraft_command.clone()])
        .await;

        let _global_command = Command::create_global_command(&ctx.http, minecraft_command.clone())
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

