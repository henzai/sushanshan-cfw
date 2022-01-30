use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use worker::Response;
use worker::Result;

#[derive(Debug, Deserialize)]
pub struct Interaction {
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<ApplicationCommandInteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub token: String,
    pub version: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMember {
    pub deaf: bool,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    /// Attached User struct.
    pub user: User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub discriminator: String,
    pub username: String,
}
#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

type Snowflake = String;
#[derive(Debug, Deserialize)]
pub struct ApplicationCommandInteractionData {
    // pub id: Snowflake,
    pub name: String,
    pub resolved: Option<ApplicationCommandInteractionDataResolved>,
    options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

impl InteractionResponse {
    pub fn reply(content: String) -> InteractionResponse {
        InteractionResponse {
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionApplicationCommandCallbackData {
                tts: None,
                content: Some(content),
                flags: None,
            }),
        }
    }
    pub fn into_response(self) -> Result<Response> {
        let sss = serde_json::to_vec(&self)?;
        let res = Response::from_bytes(sss)?;
        let mut hds = worker::Headers::new();
        hds.set("content-type", "application/json")?;
        Ok(res.with_headers(hds))
    }
}

#[derive(Debug, Deserialize)]
struct ApplicationCommandInteractionDataOption {
    name: String,
    #[serde(rename = "type")]
    option_type: ApplicationCommandOptionType,
    // the value of the pair
    value: Option<String>,
    // present if this option is a group or subcommand
    options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandInteractionDataResolved {
    pub messages: Option<HashMap<String, Message>>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub content: String,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum ApplicationCommandOptionType {
    SUBCOMMAND = 1,
    SUBCOMMANDGROUP = 2,
    STRING = 3,
    INTEGER = 4,
    BOOLEAN = 5,
    USER = 6,
    CHANNEL = 7,
    ROLE = 8,
}

#[derive(Serialize, Debug)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_response_type: InteractionResponseType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    // Acknowledge = 2,
    // ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

#[derive(Serialize, Debug)]
pub struct InteractionApplicationCommandCallbackData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    // embeds
    // allowed_mentions
    pub flags: Option<usize>,
}
