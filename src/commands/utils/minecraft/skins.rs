use super::minecraft_api_objects;
use serenity::builder::CreateAttachment;
use base64::engine::general_purpose;
use base64::Engine;

pub async fn download_face(user: String) -> Result<String,Box<dyn std::error::Error>> {
    // The recommended size for Discord avatars
    let discord_recommended_size: u32 = 108;

    // Construct the URL for the player's profile information
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", user);

    // Make a request to the Mojang API to get the player's profile
    let user_json = reqwest::get(url)
        .await?
        .json::<minecraft_api_objects::User>()
        .await?;

    // Construct the URL for the player's session data
    let session_url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}",user_json.id);

    // Make a request to the session server API to get the player's session data
    let user_session_json = reqwest::get(session_url)
        .await?
        .json::<minecraft_api_objects::SessionData>()
        .await?;

    // Get the base64-encoded version of the skin data
    let skin_object_base64 = &user_session_json.properties[0].value;

    // Decode the base64-encoded skin data into a vector of bytes
    let skin_object_vec_u8 = general_purpose::STANDARD.decode(skin_object_base64)?;

    // Convert the vector of bytes into a string
    let texture_json = std::str::from_utf8(&skin_object_vec_u8)?;

    // Deserialize the JSON string into an `EncodedTextureObject`
    let minecraft_texture_object: minecraft_api_objects::EncodedTextureObject = serde_json::from_str(texture_json)?;

    // Get the URL of the skin image from the `EncodedTextureObject`
    let skin_url = minecraft_texture_object.textures.SKIN.url;

    // Make a request to download the skin image
    let img_bytes = reqwest::get(skin_url).await?
        .bytes().await?;

    // Load the image from the vector of bytes
    let mut image = image::load_from_memory(&img_bytes)?;

    // Crop the image to only include the player's face
    let sub_image = image::imageops::crop(&mut image, 8, 8, 8, 8);

    // Construct the path where the image will be saved
    let path = format!("skin_face_{}.png", user);

    // Resize the image to the recommended size for Discord
    let resized_image = image::imageops::resize(&sub_image.to_image(), discord_recommended_size, discord_recommended_size, image::imageops::FilterType::Nearest);

    // Save the resized image to the file system
    resized_image.save_with_format(&path, image::ImageFormat::Png)
        .expect("Expected image to save to the filesystem");

    // Construct the formatted path to the image on the file system
    let formatted_path = format!("./{}",path);

    let attachment_builder = CreateAttachment::path(formatted_path)
        .await
        .expect("Expected CreateAttachment object to be made from the Path");

    let base64 = CreateAttachment::to_base64(&attachment_builder);

    Ok(base64)
}


