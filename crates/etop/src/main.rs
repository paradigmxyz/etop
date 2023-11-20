// pub mod action;
// pub mod app;
// pub mod cli;
// pub mod components;
// pub mod config;
// pub mod tui;
// pub mod utils;
use toolstr::format;

// fn main() {}
fn main() {
    // let as_str = format_num(".5", 6518);
    // let as_str = format_num(".2s", 0.012345);
    match format("", 123) {
        Ok(as_str) => println!("{}", as_str),
        Err(e) => println!("error {}", e),
    }
}

// use clap::Parser;
// use cli::Cli;
// use color_eyre::eyre::Result;

// use crate::{
//   app::App,
//   utils::{initialize_logging, initialize_panic_handler, version},
// };

// async fn tokio_main() -> Result<()> {
//   initialize_logging()?;

//   initialize_panic_handler()?;

//   let args = Cli::parse();
//   let mut app = App::new(args.tick_rate, args.frame_rate)?;
//   app.run().await?;

//   Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//   if let Err(e) = tokio_main().await {
//     eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
//     Err(e)
//   } else {
//     Ok(())
//   }
// }
