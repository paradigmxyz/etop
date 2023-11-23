mod cli;
mod dfs;

pub use cli::EtopError;

fn main() {
    cli::run_cli()
}
