use crate::Cli;
use etop_core::{EtopError, EtopState, Window, WindowSize};

const DEFAULT_DATASET: &str = "transactions_by_to_address";

pub(crate) async fn tui_command(args: Cli) -> Result<(), EtopError> {
    let etop_state =
        create_etop_state(args.dataset, args.block, args.window, args.rpc, args.data_dir).await?;

    // run main function
    etop_tui::tokio_main(Some(etop_state))
        .await
        .map_err(|e| EtopError::TuiError(format!("{:?}", e)))
        .ok();
    Ok(())
}

/// create etop state
pub(crate) async fn create_etop_state(
    dataset: Option<String>,
    block: Option<u32>,
    window_size: Option<String>,
    rpc_url: Option<String>,
    data_dir: Option<String>,
) -> Result<EtopState, EtopError> {
    // create Window
    let window = create_window(block, window_size)?;

    // create data sources
    let file_source = data_dir.clone();
    let rpc_source = create_rpc_source(rpc_url).await?;

    // crate state
    let state = EtopState {
        window,
        dataset: dataset.unwrap_or(DEFAULT_DATASET.to_string()),
        file_source,
        rpc_source,
        ..Default::default()
    };
    Ok(state)
}

fn create_window(block: Option<u32>, window_size: Option<String>) -> Result<Window, EtopError> {
    let end_block = block;
    let window_size = match window_size {
        Some(size) => size
            .parse::<u32>()
            .map_err(|_| EtopError::ParseError("could not parse window".to_string()))?,
        None => 1,
    };
    let start_block = end_block.map(|end_block| end_block - window_size + 1);
    let window_size = WindowSize::Block(window_size);
    Ok(Window { start_block, end_block, live: false, size: window_size })
}

async fn create_rpc_source(
    rpc_url: Option<String>,
) -> Result<Option<std::sync::Arc<cryo_freeze::Source>>, EtopError> {
    match parse_rpc_url(rpc_url) {
        Some(rpc_url) => {
            let rpc =
                cryo_freeze::Source::init(Some(rpc_url)).await.map_err(EtopError::CryoError)?;
            Ok(Some(std::sync::Arc::new(rpc)))
        }
        None => Ok(None),
    }
}

fn parse_rpc_url(rpc_url: Option<String>) -> Option<String> {
    let mut url = match rpc_url {
        Some(url) => url.clone(),
        _ => match std::env::var("ETH_RPC_URL") {
            Ok(url) => url,
            Err(_e) => return None,
        },
    };
    if !url.starts_with("http") {
        url = "http://".to_string() + url.as_str();
    };
    Some(url)
}
