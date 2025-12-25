use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, BufWriter, Stdout};

use crate::popups::popup_sending_message;
use crate::{App, Popup};

const MAX_HISTORY_MESSAGES: usize = 50;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn ai_character() -> Self {
        let character = match env::var("AI_PERSONALITY") {
            Ok(env) => env,
            Err(_) => "Speak clearly and shortly".into(),
        };

        Self {
            role: "system".into(),
            content: character,
        }
    }

    pub fn user_input(input: String) -> Self {
        Self {
            role: "user".into(),
            content: input,
        }
    }

    pub fn ai_reply(reply: String) -> Self {
        Self {
            role: "assistant".into(),
            content: reply,
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    model: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug)]
pub enum ChatError {
    EnvVar,
    Io,
    Network,
    ApiResponse,
}

impl From<io::Error> for ChatError {
    fn from(_err: io::Error) -> Self {
        ChatError::Io
    }
}

impl From<reqwest::Error> for ChatError {
    fn from(_err: reqwest::Error) -> Self {
        ChatError::Network
    }
}

pub fn manage_history(messages: &mut Vec<Message>) {
    if messages.len() > MAX_HISTORY_MESSAGES + 1 {
        let system_msg = messages[0].clone();
        let recent_messages: Vec<Message> = messages
            .iter()
            .skip(messages.len() - MAX_HISTORY_MESSAGES)
            .cloned()
            .collect();

        messages.clear();
        messages.push(system_msg);
        messages.extend(recent_messages);

        println!("Conversation history trimmed to keep recent messages\n",);
    }
}

pub fn send_chat_request(
    stdout: &mut BufWriter<Stdout>,
    app: &mut App,
) -> Result<String, ChatError> {
    let model = match env::var("AI_MODEL") {
        Ok(env) => env,
        Err(_) => return Err(ChatError::EnvVar),
    };

    let request = ChatRequest {
        model,
        messages: app.messages.to_vec(),
    };

    popup_sending_message(stdout, &app.size)?;

    let response = app
        .client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(app.api_key.clone())
        .json(&request)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());

        app.popup = Popup::Error(format!("HTTP {}: {}", status, error_text));
        return Err(ChatError::Network);
    }

    let chat_response: ChatResponse = response.json()?;

    if chat_response.choices.is_empty() {
        app.popup = Popup::Error("No response choices received".into());
        return Err(ChatError::ApiResponse);
    }

    Ok(chat_response.choices[0].message.content.clone())
}
