use std::env;

mod skins;
mod minecraft_api_objects;

use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                "minecraftemote" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected minecraft_username option")
                        .resolved
                        .as_ref()
                        .expect("Expected minecraft_username String Object");

                    let user_permissions_valid = command
                        .member
                        .clone()
                        .expect("Expect member to be there")
                        .permissions
                        .expect("Expect the permissions Object to be there")
                        .manage_emojis_and_stickers();

                    let bot_permissions_valid = command
                        .app_permissions
                        .expect("Expect the app_permissions Object to be there")
                        .manage_emojis_and_stickers();

                    let guild = command
                        .guild_id
                        .expect("Expect GuildID");

                    if !bot_permissions_valid {
                      "I do not have the Manage Emoji and Stickers permission in my role, please have your admin add them".to_string()
                    } else if !user_permissions_valid {
                        "You do no have Emoji editing permissions".to_string()
                    } else if let CommandDataOptionValue::String(minecraft_username) = options {
                        let local_emote_path = skins::download_face(minecraft_username.clone())
                            .await
                            .expect("Expect Skin Face to Download");

                        let emoji_name = format!("{}Minecraft",minecraft_username);

                        let emoji_face = GuildId::create_emoji(guild, &ctx, &emoji_name, &local_emote_path).await;

                        match emoji_face {
                            Ok(emoji_obj) => format!("Emoji created as {}", emoji_obj.name),
                            Err(err) => panic!("{}",err),
                        }
                    } else {
                        "Issue parsing minecraft_username".to_string()
                    }

                },
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("minecraftemote")
                        .description("Create an Emote for the Server based on a Minecraft User's Skin")
                        .create_option(|option| {
                            option
                                .name("minecraft_username")
                                .description("The Minecraft Username of the User you want to make an Emote of")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command.name("id").description("Get a user id").create_option(|option| {
                        option
                            .name("id")
                            .description("The user to lookup")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                })
        })
        .await;

        // println!("I now have the following guild slash commands: {:#?}", _commands);

        let _global_command = Command::create_global_application_command(&ctx.http, |command| {
                command
                    .name("minecraftemote")
                    .description("Create an Emote for the Server based on a Minecraft User's Skin")
                    .create_option(|option| {
                        option
                            .name("minecraft_username")
                            .description("The Minecraft Username of the User you want to make an Emote of")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
        })
        .await;

        // println!("I created the following global slash command: {:#?}", _global_command);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

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

