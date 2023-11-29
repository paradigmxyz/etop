//! etop cli
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

mod args;
mod commands;

pub use args::{Cli, Commands, DataframeArgs, DatasetArgs, NumberArgs, TuiArgs};
use clap::Parser;
use commands::{dataframe_command, dataset_command, number_command, tui_command};
pub use etop_core::EtopError;

#[tokio::main]
async fn main() -> Result<(), EtopError> {
    run_cli().await
}

pub(crate) async fn run_cli() -> Result<(), EtopError> {
    match Cli::parse().command {
        Commands::Number(args) => number_command::number_command(args),
        Commands::Dataframe(args) => dataframe_command::dataframe_command(args),
        Commands::Dataset(args) => dataset_command::dataset_command(args),
        Commands::Tui(args) => tui_command::tui_command(args).await,
    }
}
