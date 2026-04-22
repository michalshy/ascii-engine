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