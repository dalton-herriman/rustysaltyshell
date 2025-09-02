use std::io::{self, Write};
use std::process::Command;
use std::collections::HashMap;
use std::path::PathBuf;

use colored::*; // For colored text

struct Shell {
    cwd: PathBuf,                       // Current working directory
    last_status: i32,                   // Last command exit status
    env_vars: HashMap<String, String>,  // Environment variables
    history: Vec<String>,               // Command history
}

impl Shell {
    fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            last_status: 0,
            env_vars: std::env::vars().collect(),
            history: Vec::new(),
        }
    }

    fn get_prompt(&self) -> String {
        format!(
            "{} {} {} {} ",
            "shell".blue().bold(),
            "➤".green(),
            self.cwd.display().to_string().yellow(),
            ":>".cyan()
        )
    }

    fn change_dir(&mut self, target: Option<&str>) {
        let new_path = match target {
            Some(path) => PathBuf::from(path),
            None => dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
        };

        if let Err(e) = std::env::set_current_dir(&new_path) {
            eprintln!("{} {}", "cd:".red(), e);
        } else {
            self.cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        }
    }
}

fn main() {
    let mut shell = Shell::new();

    loop {
        print!("{}", shell.get_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("{}", "Failed to read line".red());
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        shell.history.push(input.to_string());

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // Built-in command: cd
        if cmd == "cd" {
            shell.change_dir(args.get(0).copied());
            continue;
        }
        
        let status = Command::new(cmd)
            .args(&args)
            .status();

        match status {
            Ok(status) => {
                shell.last_status = status.code().unwrap_or(-1);
                if !status.success() {
                    eprintln!(
                        "{} {}",
                        "Command exited with status:".red(),
                        shell.last_status
                    );
                }
            }
            Err(e) => {
                shell.last_status = -1;
                eprintln!("{} {}", "Failed to execute command:".red(), e);
            }
        }
    }
}
