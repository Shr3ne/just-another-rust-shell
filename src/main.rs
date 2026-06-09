// src/main.rs
use std::io::{self, Write};

// Register your two sub-files as modules
mod display;
mod cmds;

fn main() {
    display::set_logo();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.is_empty() {
            continue;
        }

        let mut words = input.split_whitespace();
        let cmd = words.next().unwrap();
        let args = words;

        // Route the commands cleanly to our modules
        match cmd {
            "cd" => cmds::handle_cd(args),
            "pwd" => cmds::handle_pwd(),
            "exit" | "end" => return,
            external_cmd => {
                cmds::execute_external(external_cmd, args);
            }
        }        
    }
}