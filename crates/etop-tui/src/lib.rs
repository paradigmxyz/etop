#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// ANCHOR: all
pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod tui;
pub mod utils;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

use crate::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};

pub async fn tokio_main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  // let args = Cli::parse();
  let tick_rate = 1.0;
  let frame_rate = 60.0;
  let mut app = App::new(tick_rate, frame_rate)?;
  app.run().await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  if let Err(e) = tokio_main().await {
    eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
    Err(e)
  } else {
    Ok(())
  }
}
// ANCHOR_END: all
