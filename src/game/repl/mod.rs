pub enum Command {
    Go(String), // dir
    Look,
    Fight,
    Talk,
    Use(String), // item name
    Unknown(String),
    Quit,
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    match parts.as_slice() {
        ["go", dir] => Command::Go(dir.to_string()),
        ["look"] => Command::Look,
        ["fight"] => Command::Fight,
        ["talk"] => Command::Talk,
        ["use", item] => Command::Use(item.to_string()),
        ["quit"] => Command::Quit,
        _ => Command::Unknown(input.to_string()),
    }
}

pub struct Repl {
    pub input_buffer: String,
    pub history: Vec<String>,     
    pub history_offset: i32,   
}

impl Repl {
    pub fn new() -> Repl {
        return Repl { 
            input_buffer: String::new(), 
            history: Vec::new(), 
            history_offset: -1
        }
    }

    pub fn history_up(&mut self) {
        if self.history_offset == -1 {
            self.history_offset = self.history.len() as i32 - 1;
            self.get_current_history();
            return;
        }
        
        if self.history.is_empty() || self.history_offset <= 0 {
            return
        }

        self.history_offset -= 1;
        self.get_current_history();
    } 

    pub fn history_down(&mut self) {
        if self.history.is_empty() || self.history_offset == -1 {
            return
        }

        if self.history_offset == self.history.len() as i32 - 1 {
            self.history_offset = -1;
            self.input_buffer.clear();
            return
        }

        self.history_offset += 1;
        self.get_current_history();
    }

    pub fn reset_idx(&mut self) {
        self.history_offset = -1;
    }

    fn get_current_history(&mut self) {
        self.input_buffer = self.history[self.history_offset as usize].clone();
    }
}