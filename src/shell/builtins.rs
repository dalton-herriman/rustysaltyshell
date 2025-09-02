use crate::shell::core::Shell;
use std::path::PathBuf;
use colored::*;

pub fn handle_builtin(shell: &mut Shell, cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "cd" => {
            change_dir(shell, args.get(0).copied());
            true
        }
        _ => false,
    }
}

fn change_dir(shell: &mut Shell, target: Option<&str>) {
    let new_path = match target {
        Some(path) => PathBuf::from(path),
        None => dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
    };

    if let Err(e) = std::env::set_current_dir(&new_path) {
        eprintln!("{} {}", "cd:".red(), e);
    } else {
        shell.cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    }
}
