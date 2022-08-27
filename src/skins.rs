use crate::minecraft_api_objects;

pub async fn download_face(user: String) -> Result<String,Box<dyn std::error::Error>> {

    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", user);

    let user_json = reqwest::get(url)
        .await?
        .json::<minecraft_api_objects::User>()
        .await?;

    let session_url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}",user_json.id);

    let user_session_json = reqwest::get(session_url)
        .await?
        .json::<minecraft_api_objects::SessionData>()
        .await?;

    let skin_object_base64 = &user_session_json.properties[0].value;

    let skin_object_vec_u8 = base64::decode(skin_object_base64)?;

    let texture_json = std::str::from_utf8(&skin_object_vec_u8)?;

    let minecraft_texture_object: minecraft_api_objects::EncodedTextureObject = serde_json::from_str(texture_json)?;

    let skin_url = minecraft_texture_object.textures.SKIN.url;

    let img_bytes = reqwest::get(skin_url).await?
        .bytes().await?;

    let mut image = image::load_from_memory(&img_bytes)?;
    let sub_image = image::imageops::crop(&mut image, 8, 8, 8, 8);
    let path = format!("skin_face_{}.png", user);

    let resized_image = image::imageops::resize(&sub_image.to_image(), 255, 255, image::imageops::FilterType::Nearest);

    resized_image.save_with_format(&path, image::ImageFormat::Png)
        .expect("Expected image to save to the filesystem");

    let formatted_path = format!("./{}",path);

    match serenity::utils::read_image(formatted_path) {
        Ok(base64_path) => Ok(base64_path),
        Err(error) => panic!("Error encoding image: {:#?}", error),
    }
}
