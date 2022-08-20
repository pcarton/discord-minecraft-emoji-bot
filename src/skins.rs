use image::*;
use bytes::Bytes;
use crate::minecraft_api_objects;

async fn get_minecraft_skin_bytes(url: String) -> reqwest::Result<Bytes> {
    let img_bytes = reqwest::get(url).await?
        .bytes().await?;

    Ok(img_bytes)
}

fn save_minecraft_skin_image(user: String, img_bytes: Bytes) -> Result<String,ImageError>{
    let mut image = image::load_from_memory(&img_bytes)?;
    let sub_image = imageops::crop(&mut image, 8, 8, 8, 8);
    let path = format!("skin_face_{}.png", user);

    match sub_image.to_image().save_with_format(&path, ImageFormat::Png) {
        Ok(_) => Ok(path),
        Err(error) => panic!("Error saving sub image: {:#?}", error),
    }
}

async fn get_user_skin_object_base64(username:String) -> reqwest::Result<String> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);
    
    let user_json = reqwest::get(url)
        .await?
        .json::<minecraft_api_objects::User>()
        .await?;

    let session_url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}",user_json.id);

    let user_session_json = reqwest::get(session_url)
        .await?
        .json::<minecraft_api_objects::SessionData>()
        .await?;

    Ok(user_session_json.properties[0].value.clone())
}

async fn get_user_decode_data(skin_object_base64: String) -> Result<Vec<u8>,base64::DecodeError> {
    let json_bytes = base64::decode(skin_object_base64)
        .expect("Expect Object to be decoded from base64");

    Ok(json_bytes)
}

async fn get_user_skin_url(json_bytes: Vec<u8>) -> serde_json::Result<String> {
    let texture_json = std::str::from_utf8(&json_bytes)
        .expect("Expect string from the byte Vec");

    println!("{}",texture_json);

    let minecraft_texture_object: minecraft_api_objects::EncodedTextureObject = serde_json::from_str(texture_json)?;

    println!("{}",minecraft_texture_object.textures.SKIN.url);
   Ok(minecraft_texture_object.textures.SKIN.url) 
}

pub async fn download_face(user: String) {
    let skin_object_base64 = get_user_skin_object_base64(user.clone())
        .await
        .expect("Expect base64 encoded object with skin url");


    let skin_object_vec_u8 = get_user_decode_data(skin_object_base64)
        .await
        .expect("Expect object to be decoded to Vec<u8>");

    let skin_url = get_user_skin_url(skin_object_vec_u8)
        .await
        .expect("Expect url for skin download");



    let img_bytes = get_minecraft_skin_bytes(skin_url)
        .await
        .expect("Expect bytes from skin url to use to turn into image");

    save_minecraft_skin_image(user, img_bytes)
        .expect("expect image to be saved");
}
