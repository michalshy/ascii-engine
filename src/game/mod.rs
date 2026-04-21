mod map;
mod room;
mod repl;
mod stats;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, layout::{Constraint, Layout, Margin, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Paragraph}};
use std::{io, time::Duration};
use map::Map;
use room::Room;
use stats::Stats;
use repl::Repl;

pub struct Game {
    map: Map,
    repl: Repl,
    stats: Stats,    
    running: bool,

    // repl
    input_buffer: String,
    history: Vec<String>,     
    history_offset: usize,     
}

impl Game {
    pub fn new(path: &str) -> Game {
        let loaded_map = Map::load(path).expect("Map didn't load correctly");
        
        Game { map: loaded_map, repl: Repl{}, stats: Stats{}, running: true, input_buffer: String::new(), history: Vec::new(), history_offset: 0 }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        while self.running {
            terminal.draw(|frame| {
                self.render(frame);
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.running = false,
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(8),
        ])
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(30),
        ])
        .split(vertical[0]);

        self.render_map(frame, horizontal[0]);
        self.render_stats(frame, horizontal[1]);
        self.render_repl(frame, vertical[1]);
    }

    fn render_map(&self, frame: &mut Frame, area: Rect) {
        self.map.render(frame, area);
        frame.render_widget(
            Block::bordered().title(" Map "),
            area,
        );
    }

    fn render_stats(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Block::bordered().title(" Statistics "),
            area,
        );
    }
    

     
    fn render_repl(&self, frame: &mut Frame, area: Rect) {
        let inner = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area.inner(Margin { horizontal: 1, vertical: 1 }));
    
        // Historia komend
        let visible = inner[0].height as usize;
        let skip = self.history.len().saturating_sub(visible);
        let lines: Vec<Line> = self.history[skip..]
            .iter()
            .map(|s| Line::from(s.as_str()))
            .collect();
        frame.render_widget(Paragraph::new(lines), inner[0]);
    
        // Input z kursorem
        let input = Line::from(vec![
            Span::raw("> "),
            Span::raw(&self.input_buffer),
            Span::styled("█", Style::default().fg(Color::Yellow)),
        ]);
        frame.render_widget(Paragraph::new(input), inner[1]);
    
        frame.render_widget(Block::bordered().title(" Command "), area);
    }
}