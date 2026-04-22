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
    pub history_offset: usize,   
}

impl Repl {
    pub fn new() -> Repl {
        return Repl { 
            input_buffer: String::new(), 
            history: Vec::new(), 
            history_offset: 0
        }
    }
}