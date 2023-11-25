mod cli;
mod datasets;
mod types;

pub(crate) use cli::*;
pub use types::{ColumnFormat, DataFrameFormat, Dataset, EtopError};

fn main() {
    match cli::run_cli() {
        Ok(_) => {},
        Err(e) => eprintln!("ERROR: {:?}", e),
    }
}
