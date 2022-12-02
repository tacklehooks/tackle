use cli::run_cli;

mod cli;
mod errors;
mod hooks;
mod package;
mod project;
mod util;

fn main() {
    run_cli();
}
