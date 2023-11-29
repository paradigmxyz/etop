use etop_core::EtopError;
use crate::TuiArgs;
// use etop_tui;


pub(crate) async fn tui_command(_args: TuiArgs) -> Result<(), EtopError> {
    etop_tui::tokio_main().await.map_err(|e| EtopError::TuiError(format!("{:?}", e))).ok();
    Ok(())
}

