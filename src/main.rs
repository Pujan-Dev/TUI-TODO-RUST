use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self};

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut todos: Vec<String> = vec![];
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(5), Constraint::Length(3)])
                .split(size);

            let todo_text = if todos.is_empty() {
                "No TODOs yet".into()
            } else {
                todos.join("\n")
            };
            let todo_widget = Paragraph::new(todo_text)
                .block(Block::default().title("📋 TODOs").borders(Borders::ALL));
            f.render_widget(todo_widget, chunks[0]);

            let input_widget = Paragraph::new(input.as_str()).block(
                Block::default()
                    .title("✍️ Add TODO (Enter to save, q to quit)")
                    .borders(Borders::ALL),
            );
            f.render_widget(input_widget, chunks[1]);

            f.set_cursor(chunks[1].x + input.len() as u16 + 1, chunks[1].y + 1);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('Q') => break,
                    KeyCode::Enter => {
                        if !input.trim().is_empty() {
                            todos.push(input.trim().to_string());
                            input.clear();
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}
