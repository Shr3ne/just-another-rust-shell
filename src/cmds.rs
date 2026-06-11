use std::env;
use std::path::Path;
use std::str::SplitWhitespace;
use std::process::{Command, Stdio};
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

pub fn execute_pipelines(pip_stages: Vec<String>) {
    let mut previous_stdout: Option<Stdio> = None;
    let mut children= Vec::new();
    let total_stages = pip_stages.len();

    for (i, stage) in pip_stages.iter().enumerate() {
        let mut words = stage.split_whitespace();
        let cmd = match words.next() {
            Some(c) => c,
            None => continue,
        };

        let args: Vec<&str> = words.collect();
        let mut command = Command::new(cmd);
        command.args(&args);

        if let Some(stdin_source) = previous_stdout.take() {
            command.stdin(stdin_source);
        }

        if i < total_stages - 1 {
            command.stdout(Stdio::piped());
        }

        match command.spawn() {
            Ok(mut child) => {
                if i < total_stages - 1 {
                    if let Some(stdout) = child.stdout.take() {
                        previous_stdout = Some(Stdio::from(stdout));
                    }
                }
                children.push(child);
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    eprintln!("Pipeline command not found: '{cmd}'");
                } else {
                    eprintln!("Pipeline failed to execute: {e}");
                }
                return;
            }
        }

        
    }

    for mut child in children {
        let _ = child.wait();
    }
}