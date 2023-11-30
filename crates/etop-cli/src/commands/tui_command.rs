use crate::TuiArgs;
use etop_core::EtopError;
use etop_core::{EtopState, FileSource, Window, WindowSize};
use ethers::prelude::*;

const DEFAULT_DATASET: &str = "transactions_by_to_address";

pub(crate) async fn tui_command(args: TuiArgs) -> Result<(), EtopError> {
    let end_block = args.block;
    let window_size = match &args.window {
        Some(size) => {
            size
                .parse::<u32>()
                .map_err(|_| EtopError::ParseError("could not parse window".to_string()))?
        },
        None => 1,
    };
    let start_block = end_block.map(|end_block| end_block - window_size + 1);
    let window_size = WindowSize::Block(window_size);


    let window = Window { start_block, end_block, live: false, size: window_size };
    let file_source = FileSource { data_dir: args.data_dir.clone() };

    // let provider = match parse_rpc_url(&args) {
    //     Some(rpc_url) => {
    //         let max_retries = 5;
    //         let initial_backoff = 500;
    //         let provider = Provider::<RetryClient<Http>>::new_client(&rpc_url, max_retries, initial_backoff)
    //                 .map_err(|_e| EtopError::ParseError("could not connect to provider".to_string()))?;
    //         Some(std::sync::Arc::new(provider))
    //     },
    //     None => None,
    // };

    let rpc_source = match parse_rpc_url(&args) {
        Some(rpc_url) => {
            let rpc = cryo_freeze::Source::init(Some(rpc_url)).await.map_err(EtopError::CryoError)?;
            Some(std::sync::Arc::new(rpc))
        },
        None => None,
    };

    let data = EtopState {
        window,
        dataset: args.dataset.unwrap_or(DEFAULT_DATASET.to_string()),
        file_source,
        rpc_source,
        ..Default::default()
    };
    etop_tui::tokio_main(Some(data))
        .await
        .map_err(|e| EtopError::TuiError(format!("{:?}", e)))
        .ok();
    Ok(())
}

fn parse_rpc_url(args: &TuiArgs) -> Option<String> {
    let mut url = match &args.rpc {
        Some(url) => url.clone(),
        _ => match std::env::var("ETH_RPC_URL") {
            Ok(url) => url,
            Err(_e) => {
                return None
            }
        },
    };
    if !url.starts_with("http") {
        url = "http://".to_string() + url.as_str();
    };
    Some(url)
}
