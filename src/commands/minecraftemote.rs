use serenity::builder::{CreateCommand,CreateCommandOption};
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::permissions::Permissions;
use serenity::model::application::{CommandInteraction,CommandOptionType,CommandDataOptionValue};
use serenity::model::id::GuildId;

fn check_permissions(member: Option<Box<Member>>, app_permissions: Option<Permissions>) -> bool {
    let user_permissions_valid = member
        .expect("Expect member to be there")
        .permissions
        .expect("Expect the permissions Object to be there")
        .manage_guild_expressions();

    let bot_permissions_valid = app_permissions
        .expect("Expect the app_permissions Object to be there")
        .manage_guild_expressions();

    user_permissions_valid && bot_permissions_valid
}

pub async fn run(ctx: &Context,command: &CommandInteraction) -> String {
    let options = &command
        .data
        .options
        .get(0)
        .expect("Expected minecraft_username option")
        .value;

    let member = command.member.clone();
    let app_permissions = command.app_permissions.clone();

    if command.guild_id.is_none() {
        "This command cannot be executed in a direct message".to_string()
    // Check if the bot and user have the required permissions
    } else if !check_permissions(member,app_permissions) {
        "I do not have the Manage Emoji and Stickers permission in my role, please have your admin add them".to_string()
    } else if let CommandDataOptionValue::String(minecraft_username) = options {
        let local_emote_path = super::utils::minecraft::skins::download_face(minecraft_username.clone())
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
}

pub fn register() -> CreateCommand {
    CreateCommand::new("minecraftemote")
        .description("Create an Emote for the Server based on a Minecraft User's Skin")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "minecraft_username", "The Minecraft Username of the User you want to make an Emote of")
                .required(true)
        )
}