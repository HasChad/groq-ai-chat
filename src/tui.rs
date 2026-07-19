use ratatui::{
    Frame,
    layout::Margin,
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        TitlePosition,
    },
};
use ratatui_textarea::WrapMode;

use crate::{App, app::Popup, input::MAX_INPUT_LENGTH, popups::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    app.get_layout(frame);

    frame.render_widget(
        app.wrapped_msg
            .clone()
            .fg(Color::White)
            .scroll((app.scroll, 0))
            .block(
                Block::new()
                    .fg(app.settings.colors.chat_color)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" Chat ")
                    .title_position(TitlePosition::Top),
            ),
        app.top_area,
    );

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).track_symbol(Some("┇"));

    let result: usize = match (app.top_area.height * app.scroll).checked_div(app.max_scroll) {
        Some(val) => val as usize,
        None => 0,
    };
    let vertical_scroll = app.scroll as usize + result;
    let mut scrollbar_state = ScrollbarState::new((app.max_scroll + app.top_area.height) as usize)
        .position(vertical_scroll);

    frame.render_stateful_widget(
        scrollbar,
        app.top_area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );

    app.textarea.set_block(
        Block::new()
            .fg(app.settings.colors.message_color)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(format!(
                " Message | {}/{} ",
                app.textarea.lines().join("\n").len(),
                MAX_INPUT_LENGTH,
            ))
            .title_position(TitlePosition::Top),
    );
    app.textarea.set_style(Style::default().fg(Color::White));
    app.textarea.set_wrap_mode(WrapMode::Word);
    app.textarea.set_cursor_line_style(Style::default());
    frame.render_widget(&app.textarea, app.bottom_area);

    match &app.popup {
        Popup::Welcome => popup_welcome(frame),
        Popup::Help => popup_help(frame),
        Popup::Status => popup_status(frame, &app.messages),
        Popup::Quit => popup_quit(frame),
        Popup::SendingMessage => popup_sending_message(frame),
        // Popup::Info(msg) => popup_info(frame, msg),
        Popup::Error(msg) => popup_error(frame, msg.as_str()),
        Popup::None => {}
    }
}

pub fn screen_size_warning(frame: &mut Frame) {
    let lines = vec![
        Line::default().spans(["Terminal size too small! "]),
        Line::default().spans([format!(
            "Width: {}, Height: {}",
            frame.area().width,
            frame.area().height
        )]),
        Line::default(),
        Line::default().spans(["Set your terminal size to minimum"]),
        Line::default().spans(["Width: 80, Height: 20"]),
    ];

    let text = Text::from(lines);
    let p = Paragraph::new(text).centered();

    frame.render_widget(p, frame.area());
}
