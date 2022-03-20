use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) type_field: i8,
    pub(crate) data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) tts: bool,
    pub(crate) content: String,
    pub(crate) embeds: Vec<String>,
    pub(crate) allowed_mentions: AllowedMentions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct AllowedMentions {
    pub(crate) parse: Vec<String>,
}
