use std::{collections::HashMap, fs};

use ratatui::{
    Frame, 
    layout::{Constraint, Layout, Margin, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Paragraph, Wrap}};

use crate::game::{map::Map, repl::Repl, room::Room, stats::Stats, story::Story};

#[derive(PartialEq, Eq)]
enum Perspective {
    Room,
    Map,
}

pub struct Renderer {
    perspective: Perspective,
    room_arts: HashMap<String, Vec<String>>    
}

impl Renderer {
    pub fn new() -> Renderer {
        let arts = load_room_arts();

        return Renderer{
            perspective: Perspective::Map,
            room_arts: arts,
        }
    }

    pub fn switch_perspective(&mut self) {
        self.perspective = if self.perspective == Perspective::Map { Perspective::Room } else { Perspective::Map };
            
    }

    pub fn render(
        &self, 
        frame: &mut Frame,
        map: &Map,
        stats: &Stats,
        repl: &Repl,
        story: &Story,
    ) {
        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(16),
            Constraint::Length(8),
        ])
        .split(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(25),
        ])
        .split(vertical[0]);

        let current_room = map.get_current_room().expect("Room out of bounds");

        match &self.perspective {
            Perspective::Room => {
                self.render_room(frame, horizontal[0], current_room);
            },
            Perspective::Map => {
                self.render_map(frame, horizontal[0], map, current_room);
            }
        }
        self.render_stats(frame, horizontal[1], stats);
        self.render_story(frame, vertical[1], story);
        self.render_repl(frame, vertical[2], repl);
    }

    fn render_story(&self, frame: &mut Frame, area: Rect, story: &Story) {
        let block = Block::bordered()
            .title(" Dialogue ")
            .border_style(Style::default().fg(Color::White));

        let dialog = story.get_current_dialogue();

        let paragraph = Paragraph::new(dialog.clone())
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, area);
    }

    fn render_map(&self, frame: &mut Frame, area: Rect, map: &Map, room: &Room) {
        let max_x = map.rooms.iter().map(|r| r.pos.x).max().unwrap_or(0);
        let max_y = map.rooms.iter().map(|r| r.pos.y).max().unwrap_or(0);
    
        const CELL_W: usize = 3; 
    
        let by_pos: HashMap<(i32, i32), &Room> =
            map.rooms.iter().map(|r| ((r.pos.x, r.pos.y), r)).collect();
    
        let mut lines: Vec<Line> = Vec::new();
    
        for y in 0..=max_y {
            lines.push(build_room_row(&by_pos, room, y, max_x, CELL_W));
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
                .block(Block::bordered().title(" Map ")),
            area,
        );
    }

    fn render_room(&self, frame: &mut Frame, area: Rect, room: &Room) {
        let art = self.room_arts
            .get(&room.id)
            .or_else(|| self.room_arts.get("base"))
            .map(|lines| lines.join("\n"))
            .unwrap_or_default();

        let art_lines: Vec<Line> = art.lines().map(Line::raw).collect();

        let art_h = art_lines.len() as u16;
        let art_w = art_lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;

        let inner = area.inner(Margin { horizontal: 1, vertical: 1 });

        let v_offset = inner.height.saturating_sub(art_h) / 2;
        let h_offset = inner.width.saturating_sub(art_w) / 2;

        let centered = Rect {
            x: inner.x + h_offset,
            y: inner.y + v_offset,
            width: art_w.min(inner.width),
            height: art_h.min(inner.height),
        };

        frame.render_widget(Block::bordered().title(format!(" {} ", room.name)), area);
        frame.render_widget(Paragraph::new(art_lines), centered);
    }

    fn render_stats(&self, frame: &mut Frame, area: Rect, stats: &Stats) {
        frame.render_widget(
            Block::bordered().title(" Statistics "),
            area,
        );
    }
    
    fn render_repl(&self, frame: &mut Frame, area: Rect, repl: &Repl) {
        let inner = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area.inner(Margin { horizontal: 1, vertical: 1 }));
    
        // Historia komend
        let visible = inner[0].height as usize;
        let skip = repl.history.len().saturating_sub(visible);
        let lines: Vec<Line> = repl.history[skip..]
            .iter()
            .map(|s| Line::from(s.as_str()))
            .collect();
        frame.render_widget(Paragraph::new(lines), inner[0]);
    
        // Input z kursorem
        let input = Line::from(vec![
            Span::raw("> "),
            Span::raw(&repl.input_buffer),
            Span::styled("█", Style::default().fg(Color::Yellow)),
        ]);
        frame.render_widget(Paragraph::new(input), inner[1]);
    
        frame.render_widget(Block::bordered().title(" Command "), area);
    }
}

fn load_room_arts() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let entries = fs::read_dir("assets/rooms").expect("Failed to open assets/rooms directory");

    for entry in entries.flatten() {
        let path = entry.path();

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        let content = fs::read_to_string(&path).expect("Failed to open asset file");
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

        map.insert(stem, lines);
    }

    map
}


fn build_room_row<'a>(                                    
    by_pos: &HashMap<(i32, i32), &'a Room>,               
    current: &Room,                                       
    y: i32,
    max_x: i32,                                           
    cell_w: usize,                                        
) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();

    for x in 0..=max_x {
        match by_pos.get(&(x, y)) {
            Some(room) => {
                let style = if room.id == current.id {                    
                    Style::default().fg(Color::Yellow)                    
                } else {
                    Style::default().fg(Color::White)                     
                };              
                let label = format!("[{:^width$}]", "R", width = cell_w -
                2);                                                       
                spans.push(Span::styled(label, style));

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
    by_pos: &HashMap<(i32, i32), &'a Room>,
    y: i32,
    max_x: i32,
    cell_w: usize,
) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();

    for x in 0..=max_x {
        let has_down = by_pos.get(&(x, y)).map_or(false, |room| {
            by_pos
                .get(&(x, y + 1))
                .map_or(false, |r| room.connections.contains(&r.id))
        });

        let connector = format!("{:^width$}", if has_down { "|" } else { " " }, width = cell_w);
        spans.push(Span::raw(connector));
        spans.push(Span::raw("   "));
    }

    Line::from(spans)
}