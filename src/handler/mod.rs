pub use translation::*;

use ed25519_compact::{PublicKey, Signature};

use model::{Interaction, InteractionResponse, InteractionResponseType, InteractionType, Message};
use worker::*;
mod model;
mod translation;

pub async fn handle_su_shan_shan(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Err(_e) = validate(req.clone()?, &ctx).await {
        return Response::error("unauthorized", 401);
    }
    response_interaction(req, &ctx).await
}

async fn validate(mut req: Request, ctx: &RouteContext<()>) -> Result<bool> {
    let body: Vec<u8> = req.bytes().await?;
    let hd = req.headers();

    let sig = hd
        .get("X-Signature-Ed25519")?
        .ok_or_else(|| "cannot get X-Signature-Ed25519".to_string())
        .and_then(|sig| hex::decode(sig).map_err(|e| e.to_string()))
        .and_then(|mm| Signature::from_slice(&mm).map_err(|e| e.to_string()))?;

    let timestamp = hd
        .get("X-Signature-Timestamp")?
        .ok_or("cannot get X-Signature-Timestamp")?;
    let content = vec![timestamp.as_bytes(), &body].concat();

    let discord_pub_key = ctx.secret("DISCORD_PUBLIC_KEY")?.to_string();
    let discord_pub_key = hex::decode(discord_pub_key).map_err(|_e| "cannot decode token")?;

    let pk = PublicKey::from_slice(&discord_pub_key).map_err(|_e| "cannot get pub key")?;

    pk.verify(content, &sig)
        .map_err(|e| Error::RustError(e.to_string()))?;

    Ok(true)
}

async fn response_interaction(mut req: Request, ctx: &RouteContext<()>) -> Result<Response> {
    let i = bind_interaction(&mut req).await?;
    match i.interaction_type {
        InteractionType::Ping => InteractionResponse {
            interaction_response_type: InteractionResponseType::Pong,
            data: None,
        }
        .into_response(),
        InteractionType::ApplicationCommand => {
            // console_log!("{:?}", i);
            let messages = i.data.unwrap().resolved.unwrap().messages.unwrap();
            let msgs: Vec<&Message> = messages.iter().map(|(_key, msg)| msg).collect();
            let msg = msgs[0];

            let deepl_api_key = ctx.secret("DEEPL_API_KEY")?.to_string();

            console_log!("{:?}\n", "aaaaaa");

            let translator = translation::Translator::new(&deepl_api_key);
            let text = translator.translate(&msg.content).await?;

            console_log!("{:?}\n", "bbbbbbb");

            InteractionResponse::reply(format!("{}\n>{}", &msg.content, text)).into_response()
        }
    }
}

async fn bind_interaction(req: &mut Request) -> Result<Interaction> {
    let byte = req.bytes().await.map_err(|e| e.to_string())?;
    let i = serde_json::from_slice::<Interaction>(byte.as_ref()).map_err(|e| e.to_string())?;
    Ok(i)
}
