mod cli;
mod dfs;
mod exceptions;

pub use exceptions::EtopError;
pub use dfs::ColumnFormat;

fn main() {
    match cli::run_cli() {
        Ok(_) => {},
        Err(e) => eprintln!("ERROR: {:?}", e),
    }
}
