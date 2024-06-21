use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct User {
    // The player's username
    pub name: String,
    // The player's unique Mojang ID
    pub id: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SessionData {
    // The player's unique Mojang ID
    pub id: String,
    // The player's username
    pub name: String,
    // A list of session properties, such as the player's skin data
    pub properties: Vec<SessionPropertiesObject>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SessionPropertiesObject {
    // The name of the property, such as "textures" for the player's skin data
    pub name: String,
    // The value of the property, such as the base64-encoded skin data
    pub value: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct EncodedTextureObject {
    // The player's unique Mojang ID
    pub profileId: String,
    // The player's username
    pub profileName: String,
    // Information about the player's skin, such as the URL of the skin image
    pub textures: TextureObject,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TextureObject {
   // Information about the player's skin, such as the URL of the skin image
   pub SKIN:  SkinObject,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SkinObject {
    // The URL of the player's skin image
    pub url: String,
}
