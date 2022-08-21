use bytes::Bytes;
use crate::minecraft_api_objects;
use std::fmt::Display;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
enum SkinFetchError {
    ImageError(image::ImageError),
    RequestError(reqwest::Error),
    Base64Error(base64::DecodeError),
}

impl Display for SkinFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinFetchError::ImageError(image_error) => 
                write!(f, "{}", image_error),
            SkinFetchError::Base64Error(base64_error) =>
                write!(f, "{}", base64_error),
            SkinFetchError::RequestError(request_error) =>
                write!(f, "{}", request_error),
        }
    }
}

impl std::error::Error for SkinFetchError {}

impl From<image::ImageError> for SkinFetchError {
    fn from(err: image::ImageError) -> Self {
        SkinFetchError::ImageError(err)
    }
}

impl From<reqwest::Error> for SkinFetchError {
    fn from(err: reqwest::Error) -> Self {
        SkinFetchError::RequestError(err)
    }
}

impl From<base64::DecodeError> for SkinFetchError {
    fn from(err: base64::DecodeError) -> Self {
        SkinFetchError::Base64Error(err)
    }
}

async fn get_minecraft_skin_bytes(url: String) -> reqwest::Result<Bytes> {
    let img_bytes = reqwest::get(url).await?
        .bytes().await?;

    Ok(img_bytes)
}

fn save_minecraft_skin_image(user: String, img_bytes: Bytes) -> Result<String,SkinFetchError>{
    let mut image = image::load_from_memory(&img_bytes)?;
    let sub_image = image::imageops::crop(&mut image, 8, 8, 8, 8);
    let path = format!("skin_face_{}.png", user);

    let resized_image = image::imageops::resize(&sub_image.to_image(), 255, 255, image::imageops::FilterType::Nearest);

    match resized_image.save_with_format(&path, image::ImageFormat::Png) {
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
