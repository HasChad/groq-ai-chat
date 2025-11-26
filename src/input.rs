use crossterm::{
    cursor::MoveLeft,
    event::{KeyCode, KeyEvent},
    queue,
};
use std::io::Stdout;

use crate::{
    App, Popup,
    ai_logic::{ChatError, MAX_INPUT_LENGTH, Message, manage_history, send_chat_request},
};

pub fn input_controller(stdout: &mut Stdout, event: KeyEvent, app: &mut App) {
    if app.popup != Popup::None {
        if event.code == KeyCode::Char('q') {
            app.popup = Popup::None;
        }
    } else {
        match event.code {
            KeyCode::Char(c) => app.input.push(c),
            KeyCode::Enter => process_input(stdout, app),
            KeyCode::Backspace => {
                if app.input.len() != 0 {
                    app.input.pop();
                }
            }
            KeyCode::Left => {
                let _ = queue!(stdout, MoveLeft(1));
            }
            KeyCode::Esc => app.run = false,
            _ => (),
        }
    }
}

pub fn process_input(stdout: &mut Stdout, app: &mut App) {
    if app.input.is_empty() {
        return;
    }

    match app.input.to_lowercase().as_str() {
        "exit" | "quit" => {
            app.run = false;
            return;
        }
        "clear" => {
            let system_msg = app.messages[0].clone();
            app.messages = vec![system_msg];
            app.input.clear();
            return;
        }
        "help" => {
            app.popup = Popup::Help;
            app.input.clear();
            return;
        }
        "status" => {
            app.popup = Popup::Status;
            app.input.clear();
            return;
        }
        _ => {}
    }

    app.messages.push(Message::user_input(app.input.clone()));
    app.input.clear();
    manage_history(&mut app.messages);

    match send_chat_request(stdout, app) {
        Ok(reply) => app.messages.push(Message::ai_reply(reply)),
        Err(ChatError::Network) => {
            app.popup = Popup::Error("Network error: Please check your internet connection.".into())
        }
        Err(ChatError::ApiResponse) => {
            app.popup = Popup::Error("API error: Please check your API key and try again.".into())
        }
        Err(ChatError::InputTooLong) => {
            app.popup = Popup::Error(format!(
                "Input too long (max {} characters)",
                MAX_INPUT_LENGTH
            ))
        }
        Err(_) => app.popup = Popup::Error("Unexpected error.".into()),
    }
}
