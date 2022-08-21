use crate::minecraft_api_objects;
use std::fmt::Display;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SkinFetchError {
    ImageError(image::ImageError),
    RequestError(reqwest::Error),
    Base64Error(base64::DecodeError),
    Utf8Error(std::str::Utf8Error),
    JsonError(serde_json::Error),
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
            SkinFetchError::Utf8Error(utf8_error) =>
                write!(f, "{}", utf8_error),
            SkinFetchError::JsonError(json_error) =>
                write!(f, "{}", json_error),
        }
    }
}

impl std::error::Error for SkinFetchError {}

impl From<image::ImageError> for SkinFetchError {
    fn from(err: image::ImageError) -> Self {
        SkinFetchError::ImageError(err)
    }
}

impl From<std::str::Utf8Error> for SkinFetchError {
    fn from(err: std::str::Utf8Error) -> Self {
        SkinFetchError::Utf8Error(err)
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

impl From<serde_json::Error> for SkinFetchError {
    fn from(err: serde_json::Error) -> Self {
        SkinFetchError::JsonError(err)
    }
}

pub async fn download_face(user: String) -> Result<String,SkinFetchError> {
    
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

    match resized_image.save_with_format(&path, image::ImageFormat::Png) {
        Ok(_) => Ok(path),
        Err(error) => panic!("Error saving sub image: {:#?}", error),
    }
}
