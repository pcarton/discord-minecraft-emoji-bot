use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct SessionData {
    pub id: String,
    pub name: String,
    pub properties: Vec<SessionPropertiesObject>,
}

#[derive(Deserialize)]
pub struct SessionPropertiesObject {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct EncodedTextureObject {
    // timestamp: integer,
    pub profileId: String,
    pub profileName: String,
    pub textures: TextureObject,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TextureObject {
   pub SKIN:  SkinObject,
}

#[derive(Deserialize)]
pub struct SkinObject {
    pub url: String,
}
