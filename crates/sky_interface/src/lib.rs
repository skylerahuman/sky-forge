use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use sky_core::TurnId;
use sky_engine::OneTurnEngine;
use sky_ledger::{LedgerBlock, LedgerSession};
use sky_providers::AnthropicProvider;

pub async fn run(initial_instruction: Option<String>) -> anyhow::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let provider = ProviderSelection::from_env()?;
    let mut app = App::new(provider.label().to_string());

    if let Some(instruction) = initial_instruction {
        app.input = instruction;
        terminal.draw(|frame| app.render(frame))?;
        send_current_input(&mut app, &provider).await;
    }

    loop {
        terminal.draw(|frame| app.render(frame))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Esc => break,
            KeyCode::Char('q') if app.input.is_empty() => break,
            KeyCode::Enter => {
                terminal.draw(|frame| app.render(frame))?;
                send_current_input(&mut app, &provider).await;
            }
            KeyCode::Backspace => app.backspace(),
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Char('j') if app.input.is_empty() => app.next(),
            KeyCode::Char('k') if app.input.is_empty() => app.previous(),
            KeyCode::Char(value) => app.input.push(value),
            _ => {}
        }
    }

    Ok(())
}

async fn send_current_input(app: &mut App, provider: &ProviderSelection) {
    let instruction = app.input.trim().to_string();
    if instruction.is_empty() {
        app.status = AppStatus::Idle;
        return;
    }

    app.status = AppStatus::Sending;
    app.blocks.clear();
    app.selected = 0;

    match provider.run(instruction.clone()).await {
        Ok(session) => {
            app.blocks = session.blocks();
            app.input.clear();
            app.status = AppStatus::Complete;
        }
        Err(error) => {
            let mut session = LedgerSession::new(instruction);
            let turn = TurnId::generate();
            session.append_user_instruction(turn);
            session.append_error(turn, error.to_string());
            app.blocks = session.blocks();
            app.status = AppStatus::Error;
        }
    }
}

enum ProviderSelection {
    Anthropic(AnthropicProvider),
    Fake,
}

impl ProviderSelection {
    fn from_env() -> anyhow::Result<Self> {
        Ok(match AnthropicProvider::from_env()? {
            Some(provider) => Self::Anthropic(provider),
            None => Self::Fake,
        })
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Anthropic(_) => "Anthropic/Meridian",
            Self::Fake => "FakeProvider",
        }
    }

    async fn run(&self, instruction: String) -> anyhow::Result<LedgerSession> {
        match self {
            Self::Anthropic(provider) => {
                OneTurnEngine::new(provider.clone()).run(instruction).await
            }
            Self::Fake => OneTurnEngine::fake().run(instruction).await,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppStatus {
    Idle,
    Sending,
    Complete,
    Error,
}

impl AppStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Sending => "sending",
            Self::Complete => "complete",
            Self::Error => "error",
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Idle => Color::Gray,
            Self::Sending => Color::Yellow,
            Self::Complete => Color::Green,
            Self::Error => Color::Red,
        }
    }
}

#[derive(Debug)]
pub struct App {
    blocks: Vec<LedgerBlock>,
    input: String,
    provider_label: String,
    selected: usize,
    status: AppStatus,
}

impl App {
    pub fn new(provider_label: impl Into<String>) -> Self {
        Self {
            blocks: Vec::new(),
            input: String::new(),
            provider_label: provider_label.into(),
            selected: 0,
            status: AppStatus::Idle,
        }
    }

    pub fn with_blocks(provider_label: impl Into<String>, blocks: Vec<LedgerBlock>) -> Self {
        Self {
            blocks,
            input: String::new(),
            provider_label: provider_label.into(),
            selected: 0,
            status: AppStatus::Complete,
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn next(&mut self) {
        if !self.blocks.is_empty() {
            self.selected = (self.selected + 1).min(self.blocks.len() - 1);
        }
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn render(&self, frame: &mut ratatui::Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title = Paragraph::new(Line::from(vec![
            Span::styled(
                "Sky Interface",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  one-turn DDD chat"),
        ]))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        let ledger = Paragraph::new(self.ledger_lines())
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title("Instruction Log")
                    .borders(Borders::ALL),
            );
        frame.render_widget(ledger, chunks[1]);

        let input = Paragraph::new(self.input.as_str())
            .wrap(Wrap { trim: false })
            .block(Block::default().title("Message").borders(Borders::ALL));
        frame.render_widget(input, chunks[2]);

        let footer = Paragraph::new(Line::from(vec![
            Span::raw("provider="),
            Span::styled(
                self.provider_label.clone(),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("  status="),
            Span::styled(
                self.status.label(),
                Style::default()
                    .fg(self.status.color())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  enter send  esc quit  empty q quit  empty j/k navigate"),
        ]))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[3]);
    }

    fn ledger_lines(&self) -> Vec<Line<'static>> {
        if self.blocks.is_empty() {
            return vec![Line::from("Type a message and press Enter.")];
        }

        self.blocks
            .iter()
            .enumerate()
            .flat_map(|(index, block)| {
                let style = if index == self.selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                vec![
                    Line::from(Span::styled(block.title.clone(), style)),
                    Line::from(block.body.clone()),
                    Line::from(""),
                ]
            })
            .collect()
    }
}

struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { terminal })
    }

    fn draw<F>(&mut self, f: F) -> io::Result<ratatui::CompletedFrame<'_>>
    where
        F: FnOnce(&mut ratatui::Frame<'_>),
    {
        self.terminal.draw(f)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_navigation_stays_in_bounds() {
        let mut app = App::with_blocks(
            "FakeProvider",
            vec![LedgerBlock { title: "A".to_string(), body: "B".to_string() }],
        );
        app.next();
        assert_eq!(app.selected, 0);
        app.previous();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn backspace_edits_input() {
        let mut app = App::new("FakeProvider");
        app.input = "abc".to_string();
        app.backspace();
        assert_eq!(app.input, "ab");
    }

    #[test]
    fn empty_ledger_prompts_for_message() {
        let app = App::new("FakeProvider");
        assert_eq!(
            app.ledger_lines()[0],
            Line::from("Type a message and press Enter.")
        );
    }
}
