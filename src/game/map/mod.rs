use serde::Deserialize;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::collections::HashMap;
use crate::game::Room;

#[derive(Deserialize)]
pub struct Map {
    rooms: Vec<Room>
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
    
        const CELL_W: usize = 3; 
    
        let by_pos: HashMap<(u32, u32), &Room> =
            self.rooms.iter().map(|r| ((r.x, r.y), r)).collect();
    
        let mut lines: Vec<Line> = Vec::new();
    
        for y in 0..=max_y {
            lines.push(build_room_row(&by_pos, y, max_x, CELL_W));
            if y < max_y {
                lines.push(build_conn_row(&by_pos, y, max_x, CELL_W));
            }
        }
    
        let grid_h = (lines.len()) as u16;
        let top_pad = area.height.saturating_sub(grid_h) / 2;
        let mut padded: Vec<Line> = vec![Line::raw(""); top_pad as usize];
        padded.extend(lines);
    
        frame.render_widget(
            Paragraph::new(padded)
                .centered()                         
                .block(Block::bordered().title(" Mapa ")),
            area,
        );
    }
    
    
}

fn build_room_row<'a>(
    by_pos: &HashMap<(u32, u32), &'a Room>,
    y: u32,
    max_x: u32,
    cell_w: usize,
) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();

    for x in 0..=max_x {
        match by_pos.get(&(x, y)) {
            Some(room) => {
                // sama komórka — jeden Span żeby terminal nie rozciągał
                let label = format!("[{:^width$}]", "R", width = cell_w - 2);
                spans.push(Span::styled(label, Style::default().fg(Color::White)));

                // connector w prawo
                let has_right = by_pos
                    .get(&(x + 1, y))
                    .map_or(false, |r| room.connections.contains(&r.id));
                spans.push(Span::raw(if has_right { "───" } else { "   " }));
            }
            None => {
                spans.push(Span::raw(" ".repeat(cell_w)));
                spans.push(Span::raw("   "));
            }
        }
    }

    Line::from(spans)
}

fn build_conn_row<'a>(
    by_pos: &HashMap<(u32, u32), &'a Room>,
    y: u32,
    max_x: u32,
    cell_w: usize,
) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();

    for x in 0..=max_x {
        let has_down = by_pos.get(&(x, y)).map_or(false, |room| {
            by_pos
                .get(&(x, y + 1))
                .map_or(false, |r| room.connections.contains(&r.id))
        });

        // connector w dół wycentrowany pod komórką
        let connector = format!("{:^width$}", if has_down { "|" } else { " " }, width = cell_w);
        spans.push(Span::raw(connector));
        spans.push(Span::raw("   ")); // wyrównanie do "───"
    }

    Line::from(spans)
}