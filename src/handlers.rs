use crate::interaction_response::{AllowedMentions, Data};
use crate::{Interaction, InteractionResponse};

pub(crate) fn handle_interaction(interaction: Interaction) -> InteractionResponse {
    match interaction.data.name.as_str() {
        "ping" => build_ping_response(),
        _ => InteractionResponse {
            type_field: 0,
            data: Default::default(),
        },
    }
}

pub(crate) fn build_ack_ping_response() -> InteractionResponse {
    InteractionResponse {
        type_field: 1,
        data: Data {
            tts: false,
            content: "PONG".to_string(),
            embeds: vec![],
            allowed_mentions: AllowedMentions { parse: vec![] },
        },
    }
}

pub(crate) fn build_ping_response() -> InteractionResponse {
    InteractionResponse {
        type_field: 4,
        data: Data {
            tts: false,
            content: "PONG".to_string(),
            embeds: vec![],
            allowed_mentions: AllowedMentions { parse: vec![] },
        },
    }
}
