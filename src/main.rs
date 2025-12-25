use crossterm::{
    cursor::Hide,
    event::{Event, read},
    execute, queue,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode,
        enable_raw_mode,
    },
};
use dotenvy::dotenv;
use std::io::{self, BufWriter, stdout};

mod ai_logic;
mod app;
mod input;
mod popups;
mod tui;

use app::*;
use input::*;
use tui::*;

fn main() -> io::Result<()> {
    dotenv().ok();
    let mut stdout = BufWriter::with_capacity(640000, stdout());
    let mut app = App::init()?;

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
