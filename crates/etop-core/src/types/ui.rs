use crate::{DataWarehouse, EtopError};
use etop_format::{ColumnFormatShorthand, DataFrameFormat};

/// ui
#[derive(Debug, Clone, Default)]
pub struct EtopState {
    /// window
    pub window: Window,
    /// other window, for comparison to main window
    pub other_window: Option<Window>,
    /// dataset being displayed
    pub dataset: String,
    /// warehouse
    pub warehouse: DataWarehouse,
    /// file source
    pub file_source: FileSource,
    /// rpc source
    pub rpc_source: RpcSource,
    // /// current df
    // pub current_df: Option<DataFrame>,
    // /// current table
    // pub current_table: Option<String>,
}

/// window
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// start block of window
    pub start_block: Option<u32>,
    /// end block of window
    pub end_block: Option<u32>,
    /// whether window is historic or updates with live data
    pub live: bool,
    /// size of window, in blocks or in time
    pub size: WindowSize,
}

/// window size
#[derive(Debug, Clone)]
pub enum WindowSize {
    /// block
    Block(u32),
    // Duration(),
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        WindowSize::Block(1)
    }
}

/// data source
#[derive(Debug, Clone, Default)]
pub enum DataSource {
    /// rpc
    Rpc(RpcSource),
    /// file
    File(FileSource),
    /// none
    #[default]
    None,
}

/// rpc source
#[derive(Debug, Clone, Default)]
pub struct RpcSource {
    // chain_id: u64,
    // rpc_provider: Option<Provider<Http>>,
}

/// file source
#[derive(Debug, Clone, Default)]
pub struct FileSource {
    /// data directory
    pub data_dir: Option<String>,
}

impl EtopState {
    /// whether any data is available to render in current window
    pub fn can_render(&self) -> bool {
        // is there any data collected for the current window?
        true
    }

    /// format data of current window
    pub fn format_window(
        &self,
        render_height: usize,
        render_width: usize,
    ) -> Result<String, EtopError> {
        let dataspec = crate::load_dataspec(self.dataset.clone())?;
        let columns = dataspec.default_columns();
        let column_formats = dataspec.default_column_formats();
        let columns: Result<Vec<ColumnFormatShorthand>, EtopError> = columns
            .iter()
            .map(|name| {
                column_formats.get(name).ok_or(EtopError::ColumnMissing(name.to_string())).cloned()
            })
            .collect::<Result<Vec<_>, _>>();
        let columns = columns?;

        let df =
            dataspec.transform(&self.warehouse, self.window.start_block, self.window.end_block)?;
        let fmt = DataFrameFormat {
            column_formats: Some(columns),
            render_height: Some(render_height - 1),
            max_render_width: Some(render_width),
            ..Default::default()
        };

        Ok(fmt.format(df)?)
    }
}
