mod shell;

use shell::core::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
