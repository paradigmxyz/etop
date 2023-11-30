use crate::TuiArgs;
use etop_core::EtopError;
use etop_core::{EtopState, FileSource, RpcSource, Window, WindowSize};

const DEFAULT_DATASET: &str = "transactions_by_to_address";

pub(crate) async fn tui_command(args: TuiArgs) -> Result<(), EtopError> {
    let start_block = None;
    let end_block = args.block;
    let window_size = match args.window {
        Some(size) => {
            let size = size
                .parse::<u32>()
                .map_err(|_| EtopError::ParseError("could not parse window".to_string()))?;
            WindowSize::Block(size)
        }
        None => WindowSize::default(),
    };
    let window = Window { start_block, end_block, live: false, size: window_size };
    let file_source = FileSource { data_dir: args.data_dir };
    let rpc_source = RpcSource {};
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
