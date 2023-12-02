use clap::Parser;

/// Utility for creating and managing MESC RPC configurations
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// dataset
    #[clap()]
    pub dataset: Option<String>,

    /// block
    #[clap(short, long)]
    pub block: Option<u32>,

    /// window size
    #[clap(short, long)]
    pub window: Option<String>,

    /// data directory
    #[clap(long, hide = true)]
    pub data_dir: Option<String>,

    /// rpc provider url
    #[clap(short, long, default_value = Some("http://localhost:8545"))]
    pub rpc: Option<String>,

    /// print formatted data without interactive interface
    #[clap(short, long)]
    pub print: bool,
}
