use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Interaction {
    pub(crate) application_id: String,
    pub(crate) channel_id: String,
    pub(crate) data: Data,
    pub(crate) guild_id: String,
    pub(crate) guild_locale: String,
    pub(crate) id: String,
    pub(crate) locale: String,
    pub(crate) member: Member,
    pub(crate) token: String,
    #[serde(rename = "type")]
    pub(crate) type_field: i64,
    pub(crate) version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Data {
    #[serde(default)]
    pub(crate) guild_id: String,
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Member {
    pub(crate) avatar: Option<String>,
    pub(crate) communication_disabled_until: Option<String>,
    pub(crate) deaf: bool,
    pub(crate) is_pending: bool,
    pub(crate) joined_at: String,
    pub(crate) mute: bool,
    pub(crate) nick: Option<String>,
    pub(crate) pending: bool,
    pub(crate) permissions: String,
    pub(crate) premium_since: Option<String>,
    pub(crate) roles: Vec<String>,
    pub(crate) user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) avatar: String,
    pub(crate) discriminator: String,
    pub(crate) id: String,
    pub(crate) public_flags: i64,
    pub(crate) username: String,
}
