mod cli;
mod dfs;
mod exceptions;
mod pipeline;

pub use exceptions::EtopError;
pub use dfs::{ColumnFormat, DataFrameFormat, Dataset};

fn main() {
    match cli::run_cli() {
        Ok(_) => {},
        Err(e) => eprintln!("ERROR: {:?}", e),
    }
}
