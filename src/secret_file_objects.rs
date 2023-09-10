use serde::Deserialize;

#[derive(Deserialize)]
pub struct SecretFile {
    pub secrets: SecretFileObj,
}

#[derive(Deserialize)]
pub struct SecretFileObj {
    pub discord_api_token: String,
    pub discord_guild_id: String,
}