mod map;
mod room;
mod repl;
mod stats;
mod render;
use ratatui::{Terminal, backend::CrosstermBackend};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use map::Map;
use room::Room;
use stats::Stats;
use repl::Repl;

use crate::game::{render::Renderer, repl::Command};



pub struct Game {
    renderer: Renderer,

    map: Map,
    repl: Repl,
    stats: Stats,    

    running: bool
}

impl Game {
    pub fn new(path: &str) -> Game {
        let loaded_map = Map::load(path).expect("Map didn't load correctly");
        Game { 
            renderer: Renderer::new(), 
            map: loaded_map, 
            repl: Repl::new(), 
            stats: Stats{}, 
            running: true
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        while self.running {
            // renderer
            terminal.draw(|frame| {
                self.renderer.render(
                    frame, &self.map, &self.stats, &self.repl,
                );
            })?;
            // animations, logic etc
            self.logic();
            // repl
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {                                          
                        KeyCode::Char(c) => self.repl.input_buffer.push(c),
                        KeyCode::Backspace => { self.repl.input_buffer.pop(); 
                    }                                                         
                        KeyCode::Enter => {
                            let input = self.repl.input_buffer.drain(..).collect::<String>();     
                            let cmd = repl::parse_command(&input);
                            self.repl.history.push(format!("> {}", input));   
                            self.handle_command(cmd);
                        }
                        KeyCode::Tab => {
                            self.renderer.switch_perspective();
                        }
                        _ => {}
                    }          
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn logic(&self) {
        
    } 

    fn handle_command(&mut self, cmd: Command) {
        match cmd {                                           
          Command::Go(dir) => { /* move player */ }
          Command::Look => { /* describe current room */ }  
          Command::Fight => { /* fight monster */ },          
          Command::Talk => { /* talk to npc */ }     
          Command::Use(item) => { /* use item */ },          
          Command::Quit => { self.running = false }     
          Command::Unknown(raw) => {                        
              self.repl.history.push(format!("Unknown       
  command: {}", raw));
          }                                                 
      }    
    }
}