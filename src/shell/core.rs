//use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use colored::*;

use crate::shell::prompt;
use crate::shell::builtins;

pub struct Shell {
    pub cwd: PathBuf,
    pub last_status: i32,
    //pub env_vars: HashMap<String, String>,
    pub history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            last_status: 0,
            //env_vars: std::env::vars().collect(),
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("{}", prompt::get_prompt(&self));
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

            self.history.push(input.to_string());

            if input == "exit" {
                break;
            }

            let mut parts = input.split_whitespace();
            let cmd = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();

            // Built-in command
            if builtins::handle_builtin(self, cmd, &args) {
                continue;
            }

            // External command
            match Command::new(cmd).args(&args).status() {
                Ok(status) => {
                    self.last_status = status.code().unwrap_or(-1);
                    if !status.success() {
                        eprintln!(
                            "{} {}",
                            "Command exited with status:".red(),
                            self.last_status
                        );
                    }
                }
                Err(e) => {
                    self.last_status = -1;
                    eprintln!("{} {}", "Failed to execute command:".red(), e);
                }
            }
        }
    }
}
