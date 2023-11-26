use super::commands::{dataframe_command, dataset_command, number_command};
use crate::EtopError;
use clap::{Parser, Subcommand};

/// Utility for creating and managing MESC RPC configurations
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Define your subcommands as an enum
#[derive(Subcommand)]
pub enum Commands {
    /// Print formatted number
    Number(NumberArgs),
    /// Print formatted dataframe file
    Dataframe(DataframeArgs),
    /// Print formatted dataset
    Dataset(DatasetArgs),
}

/// Arguments for the `number` subcommand
#[derive(Parser)]
pub struct NumberArgs {
    /// Number to format
    #[clap()]
    pub number: String,

    /// Number format
    #[clap(long)]
    pub format: Option<String>,

    /// All formats
    #[clap(long)]
    pub all_formats: bool,
}

/// Arguments for the `dataframe` subcommand
#[derive(Parser)]
pub struct DataframeArgs {
    /// Number to format
    #[clap()]
    pub path: String,

    /// Number format
    #[clap(long)]
    pub format: Option<String>,

    /// Columns to load, COLUMN_NAME[=NEW_NAME][:WIDTH]
    #[clap(long, num_args=1..)]
    pub columns: Option<Vec<String>>,

    /// Rows to print
    #[clap(long)]
    pub rows: Option<usize>,

    /// Template
    #[clap(long, num_args=1..)]
    pub template: Option<String>,
}

/// Arguments for the `dataframe` subcommand
#[derive(Parser)]
pub struct DatasetArgs {
    /// Dataset to load
    #[clap()]
    pub dataset: String,

    /// Number to format
    #[clap(long)]
    pub data_dir: Option<String>,

    /// Columns to load
    #[clap(long, num_args=1..)]
    pub columns: Option<Vec<String>>,

    /// Window size
    pub window: Option<u64>,
}

pub(crate) fn run_cli() -> Result<(), EtopError> {
    match Cli::parse().command {
        Commands::Number(args) => number_command::number_command(args),
        Commands::Dataframe(args) => dataframe_command::dataframe_command(args),
        Commands::Dataset(args) => dataset_command::dataset_command(args),
    }
}
