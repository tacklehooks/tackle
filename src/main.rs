use cli::run_cli;

mod cli;
mod errors;
mod hooks;
mod package;
mod project;
mod util;
mod graph;

fn main() {
    run_cli();
}
