//! etop cli
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

mod args;
mod commands;

pub use args::Cli;
use clap::Parser;
use commands::{print_command, tui_command};
pub use etop_core::EtopError;

#[tokio::main]
async fn main() -> Result<(), EtopError> {
    run_cli().await
}

pub(crate) async fn run_cli() -> Result<(), EtopError> {
    let args = Cli::parse();
    if args.print {
        print_command::print_command(args).await
    } else {
        tui_command::tui_command(args).await
    }
}
