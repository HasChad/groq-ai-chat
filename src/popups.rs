use std::io::{self, Stdout};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor, Stylize},
};

use crate::{ai_logic::Message, tui::draw_box_with_title};

pub fn popup_welcome(stdout: &mut Stdout, size: &(u16, u16)) -> io::Result<()> {
    let x_size = 38;
    let y_size = 6;
    let x_pos = (size.0 - x_size) / 2;
    let y_pos = (size.1 - y_size) / 2;

    draw_box_with_title(
        stdout,
        x_size,
        y_size,
        x_pos,
        y_pos,
        "Welcome".into(),
        Color::Cyan,
    )?;

    execute!(
        stdout,
        MoveTo(x_pos + 1, y_pos + 1),
        SetForegroundColor(Color::Yellow),
        Print("      Welcome to Groq AI Chat!".bold()),
        MoveTo(x_pos + 1, y_pos + 2),
        SetForegroundColor(Color::Yellow),
        Print("────────────────────────────────────"),
        SetForegroundColor(Color::Blue),
        MoveTo(x_pos + 1, y_pos + 3),
        Print("- Type 'help' for available commands"),
        MoveTo(x_pos + 1, y_pos + 4),
        Print("- Press 'p' to close pop-ups"),
        SetForegroundColor(Color::Reset),
    )?;

    Ok(())
}

pub fn popup_help(stdout: &mut Stdout, size: &(u16, u16)) -> io::Result<()> {
    let x_size = 52;
    let y_size = 7;
    let x_pos = (size.0 - x_size) / 2;
    let y_pos = (size.1 - y_size) / 2;

    draw_box_with_title(
        stdout,
        x_size,
        y_size,
        x_pos,
        y_pos,
        "Help".into(),
        Color::Blue,
    )?;

    execute!(
        stdout,
        MoveTo(x_pos + 1, y_pos + 1),
        SetForegroundColor(Color::Yellow),
        Print("Available commands:"),
        MoveTo(x_pos + 1, y_pos + 2),
        SetForegroundColor(Color::Grey),
        Print(" exit | quit   - Quit"),
        MoveTo(x_pos + 1, y_pos + 3),
        Print(" help          - Show this help message"),
        MoveTo(x_pos + 1, y_pos + 4),
        Print(" status        - Show current conversation status"),
        MoveTo(x_pos + 1, y_pos + 5),
        Print(" clear         - Clear chat history"),
        SetForegroundColor(Color::Reset),
    )?;

    Ok(())
}

pub fn popup_status(
    stdout: &mut Stdout,
    size: &(u16, u16),
    messages: &[Message],
) -> io::Result<()> {
    let x_size = 30;
    let y_size = 6;
    let x_pos = (size.0 - x_size) / 2;
    let y_pos = (size.1 - y_size) / 2;

    draw_box_with_title(
        stdout,
        x_size,
        y_size,
        x_pos,
        y_pos,
        "Status".into(),
        Color::Blue,
    )?;

    let user_messages = messages.iter().filter(|m| m.role == "user").count();
    let assistant_messages = messages.iter().filter(|m| m.role == "assistant").count();

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        MoveTo(x_pos + 1, y_pos + 1),
        Print("Conversation Status:"),
        SetForegroundColor(Color::Grey),
        MoveTo(x_pos + 1, y_pos + 2),
        Print(format!("- Messages in history: {}", messages.len() - 1,)),
        MoveTo(x_pos + 1, y_pos + 3),
        Print(format!("- User messages: {}", user_messages)),
        MoveTo(x_pos + 1, y_pos + 4),
        Print(format!("- AI responses: {}", assistant_messages)),
        SetForegroundColor(Color::Reset),
    )?;

    Ok(())
}

pub fn popup_sending_message(stdout: &mut Stdout, size: &(u16, u16)) -> io::Result<()> {
    let msg = "Sending message";

    let x_size = msg.len() as u16 + 2;
    let y_size = 3;
    let x_pos = (size.0 - x_size) / 2;
    let y_pos = (size.1 - y_size) / 2;

    draw_box_with_title(
        stdout,
        x_size,
        y_size,
        x_pos,
        y_pos,
        "Info".into(),
        Color::DarkBlue,
    )?;

    execute!(
        stdout,
        Hide,
        MoveTo(x_pos + 1, y_pos + 1),
        SetForegroundColor(Color::Yellow),
        Print(msg),
        SetForegroundColor(Color::Reset),
    )?;

    Ok(())
}

pub fn popup_error(stdout: &mut Stdout, size: &(u16, u16), msg: &str) -> io::Result<()> {
    let x_size = msg.len() as u16 + 2;
    let y_size = 3;
    let x_pos = (size.0 - x_size) / 2;
    let y_pos = (size.1 - y_size) / 2;

    draw_box_with_title(
        stdout,
        x_size,
        y_size,
        x_pos,
        y_pos,
        "Error".into(),
        Color::Red,
    )?;

    execute!(
        stdout,
        MoveTo(x_pos + 1, y_pos + 1),
        SetForegroundColor(Color::Yellow),
        Print(msg),
        SetForegroundColor(Color::Reset),
    )?;

    Ok(())
}
