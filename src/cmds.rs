use std::env;
use std::path::Path;
use std::str::SplitWhitespace;
use std::process::Command;
use std::io::ErrorKind;

pub fn handle_cd(args: SplitWhitespace) {
    let destination = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(destination);
    
    if let Err(error) = env::set_current_dir(&root) {
        eprintln!("{error}");
    }
}

pub fn handle_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Error reading current directory: {e}"),
    }
}

pub fn execute_external(cmd: &str, args: SplitWhitespace) {
    match Command::new(cmd).args(args).spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Command not found: '{cmd}'");
            } else {
                println!("Failed to execute command: '{cmd}'");
            }
        }
    }
}