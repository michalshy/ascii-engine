use crossterm::event;
use serde::Deserialize;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Map {
    rooms: Vec<Room>
}

#[derive(Deserialize)]
pub struct Room {
    id: String,
    name: String,
    x: u32,
    y: u32,
    event: Event,
    connections: Vec<String>
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Npc {
        name: String,
        dialogue: String,
    },
    Enemy {
        name: String,
        health: u32,
        damage: u32,
        description: String,
    },
    Item {
        name: String,
        description: String,
    },
    Nothing,
}

impl Map {
    pub fn load(path: &str) -> anyhow::Result<Map> {
        let content = std::fs::read_to_string(path)?;
        let map: Map = serde_json::from_str(&content)?;
        Ok(map)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let max_x = self.rooms.iter().map(|r| r.x).max().unwrap_or(0);
        let max_y = self.rooms.iter().map(|r| r.y).max().unwrap_or(0);

        let cell_w = 12usize;
        let cell_h = 3usize;

        let by_pos: HashMap<(u32, u32), &Room> = 
            self.rooms.iter().map(|r| ((r.x, r.y), r)).collect();

        let mut lines: Vec<Line> = Vec::new();

        for y in 0..=max_y {
            let mut room_row: Vec<Span> = Vec::new();
            let mut conn_row: Vec<Span> = Vec::new();

            for x in 0..=max_x {
                if let Some(room) = by_pos.get(&(x,y)) {
                    let color = event_color(&room.event);
                    let label = format!("{:^width$}", truncate(&room.name, cell_w - 2), width = cell_w - 2);
                    room_row.push(Span::styled(format!("[{}]", label), Style::default().fg(color)));

                    let right = by_pos.get(&(x + 1, y));
                    let has_right = right.map_or(false, |r| room.connections.contains(&r.id));
                    room_row.push(Span::raw(if has_right { "──" } else { "  " }));

                    let down = by_pos.get(&(x, y + 1));
                    let has_down = down.map_or(false, |r| room.connections.contains(&r.id));
                    let pad = " ".repeat(cell_w - 2);
                    conn_row.push(Span::raw(format!("  {}  ", if has_down { "|" } else { " " })));
                    conn_row.push(Span::raw("  "));
                } else {
                    room_row.push(Span::raw(" ".repeat(cell_w)));
                    room_row.push(Span::raw("  "));
                    conn_row.push(Span::raw(" ".repeat(cell_w)));
                    conn_row.push(Span::raw("  "));
                }
            }

            lines.push(Line::from(room_row));
            if y < max_y {
                lines.push(Line::from(conn_row));
            }
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }
}

fn event_color(event: &Event) -> Color {
    match event {
        Event::Enemy { .. } => Color::Black,
        Event::Npc { .. } => Color::Black,
        Event::Item { .. } => Color::Black,
        Event::Nothing { .. } => Color::Black,
    }
}

fn truncate(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}