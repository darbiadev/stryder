//! # stryder
//!
//! stryder is a Discord bot that provides utility functions for Darbia's Discord servers.

use actix_web::{error, web, App, HttpRequest, HttpServer, Responder, Result};
use ring::signature;
use serde_json::Value;

use crate::handlers::{build_ack_ping_response, handle_interaction};
use crate::interaction::Interaction;
use crate::interaction_response::InteractionResponse;

mod handlers;
mod interaction;
mod interaction_response;

async fn handle_request(req: HttpRequest, body: String) -> Result<impl Responder> {
    let peer_public_key_hex = std::env::var("STRYDER_PUBLIC_KEY").unwrap();
    let peer_public_key_bytes = hex::decode(peer_public_key_hex.as_bytes()).unwrap();
    let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);

    let timestamp = req
        .headers()
        .get("x-signature-timestamp")
        .unwrap()
        .to_str()
        .unwrap();
    let signature_hex = req
        .headers()
        .get("x-signature-ed25519")
        .unwrap()
        .to_str()
        .unwrap();
    let signature_bytes = hex::decode(signature_hex).unwrap();

    let message = format!("{}{}", timestamp, body);

    let result = peer_public_key.verify(message.as_bytes(), signature_bytes.as_slice());

    let mut interaction_response = build_ack_ping_response();
    let data: Value = serde_json::from_str(body.as_str()).unwrap();
    if data["type"].as_i64().unwrap() != 1 {
        let interaction: Interaction = serde_json::from_str(body.as_str()).unwrap();
        println!("{:#?}", interaction);
        interaction_response = handle_interaction(interaction);
    }

    if result.is_ok() {
        Ok(web::Json(interaction_response))
    } else {
        Err(error::ErrorUnauthorized(result.unwrap_err()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/stryder", web::post().to(handle_request)))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
