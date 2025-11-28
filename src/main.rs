use crossterm::{
    cursor::Hide,
    event::{Event, read},
    execute, queue,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use dotenvy::dotenv;
use reqwest::blocking::Client;
use std::{
    env,
    fs::File,
    io::{self},
    process,
};

mod ai_logic;
mod input;
mod popups;
mod tui;

use ai_logic::*;
use input::*;
use tui::*;

#[derive(PartialEq)]
pub enum Popup {
    None,
    Welcome,
    Help,
    Status,
    Error(String),
}

pub struct App {
    run: bool,
    size: (u16, u16),
    messages: Vec<Message>,
    api_key: String,
    client: Client,
    input: String,
    popup: Popup,
    file: File,
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let mut stdout = io::stdout();
    let system_message = Message::ai_character();

    let api_key = match env::var("GROQ_API_KEY") {
        Ok(env) => env,
        Err(_) => {
            println!(
                "\nGROQ_API_KEY environment variable not found. Please set it in your .env file!"
            );
            process::exit(1);
        }
    };

    let mut file = match File::open("messages.json") {
        Ok(file) => file,
        Err(_) => File::create("messages.json").unwrap(),
    };

    let mut app = App {
        run: true,
        messages: vec![system_message],
        api_key,
        client: Client::new(),
        input: String::new(),
        size: size()?,
        popup: Popup::Welcome,
        file,
    };

    execute!(
        stdout,
        SetTitle("Groq AI Chat"),
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide
    )?;
    enable_raw_mode()?;
    render(&mut stdout, &mut app)?;

    while app.run {
        match read()? {
            Event::Resize(width, height) => app.size = (width, height),
            Event::Key(event) => input_controller(&mut stdout, event, &mut app),
            _ => (),
        }

        queue!(stdout, Clear(ClearType::All), Hide)?;

        if app.size.0 < 80 || app.size.1 < 20 {
            screen_size_warning(&mut stdout, &app.size)?;
            continue;
        }

        render(&mut stdout, &mut app)?;
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
