use clap::{Parser, Subcommand};
use polars::prelude::*;
use toolstr::NumberFormat;

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
    /// Print formatted dataframe
    Dataframe(DataframeArgs),
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
}

pub(crate) fn run_cli() {
    match Cli::parse().command {
        Commands::Number(args) => number_command(args),
        Commands::Dataframe(args) => dataframe_command(args),
    }
}

/// print formatted number
fn number_command(args: NumberArgs) {
    let number: Result<f64, _> = args.number.parse();
    match number {
        Ok(number) => match (args.format, args.all_formats) {
            (Some(format), _) => print_format(number, format),
            (None, true) => print_all_formats(number),
            (None, false) => println!("provide --format or --all-formats"),
        },
        Err(_) => println!("could not parse number: {}", args.number),
    }
}

fn print_format(number: f64, format: String) {
    println!("number: {:?}", number);
    println!("format: {}", format);
    println!();
    match toolstr::format(format.as_str(), number) {
        Ok(formatted) => println!("output: {}", formatted),
        Err(e) => println!("could not format number: {:?}", e),
    }
}

fn print_all_formats(number: f64) {
    println!("number: {:?}", number);
    println!("format: all");
    println!();
    for format_type in toolstr::FormatType::all_variants() {
        let format = NumberFormat::new().format_type(&format_type);
        match format.format(number) {
            Ok(formatted) => println!("{:?}: {}", format_type, formatted),
            Err(e) => println!("could not format number: {:?}", e),
        }
    }
}

/// print dataframe command
fn dataframe_command(args: DataframeArgs) {
    println!("path: {}", args.path);
    println!("format: {:?}", args.format);
    print_dataframe(args.path)
}

#[derive(Debug)]
enum EtopError {
    CouldNotOpenFile(String),
    CouldNotReadFile(String),
}

fn print_dataframe(path: String) {
    // read file
    let df = read_parquet(path);

    // print file
    println!("{:?}", df);
}

fn read_parquet(path: String) -> Result<DataFrame, EtopError> {
    // print number of rows
    let file = std::fs::File::open(path.as_str())
        .map_err(|_| EtopError::CouldNotOpenFile(path.clone()))?;

    ParquetReader::new(file)
        // .with_columns(Some(vec![column.to_string()]))
        .finish()
        .map_err(|_| EtopError::CouldNotReadFile(path.clone()))
}
