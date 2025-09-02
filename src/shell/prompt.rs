use crate::shell::core::Shell;
use colored::*;

pub fn get_prompt(shell: &Shell) -> String {
    format!(
        "{} {} {} {} ",
        "shell".blue().bold(),
        "➤".green(),
        shell.cwd.display().to_string().yellow(),
        ":>".cyan()
    )
}
