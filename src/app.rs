use crossterm::terminal::size;
use reqwest::blocking::Client;
use std::{
    env,
    fs::{self, File},
    io, process,
};

use crate::ai_logic::Message;

pub const FILE_PATH: &str = "messages.json";

#[derive(PartialEq)]
pub enum Popup {
    None,
    Welcome,
    Help,
    Status,
    Error(String),
}

pub struct App {
    pub run: bool,
    pub size: (u16, u16),
    pub messages: Vec<Message>,
    pub api_key: String,
    pub client: Client,
    pub input: String,
    pub popup: Popup,
    pub scroll: u32,
}

impl App {
    pub fn init() -> io::Result<Self> {
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

        let messages: Vec<Message> = match File::open(FILE_PATH) {
            Ok(_) => {
                let mut json_data = fs::read_to_string(FILE_PATH).unwrap();

                if json_data.is_empty() {
                    json_data = serde_json::to_string(&vec![system_message]).unwrap();
                    fs::write(FILE_PATH, json_data.clone()).unwrap();
                }

                serde_json::from_str(&json_data).unwrap()
            }
            Err(_) => {
                File::create(FILE_PATH).unwrap();

                let json_data = serde_json::to_string(&vec![system_message]).unwrap();
                fs::write(FILE_PATH, json_data.clone()).unwrap();

                serde_json::from_str(&json_data).unwrap()
            }
        };

        Ok(Self {
            run: true,
            messages,
            api_key,
            client: Client::new(),
            input: String::new(),
            size: size()?,
            popup: Popup::None,
            scroll: 0,
        })
    }
}
