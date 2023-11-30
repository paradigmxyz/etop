use crate::{DataWarehouse, EtopError};
use etop_format::{ColumnFormatShorthand, DataFrameFormat};
use polars::prelude::*;

/// ui
#[derive(Debug, Clone, Default)]
pub struct EtopState {
    /// latest block
    pub latest_block: Option<u32>,
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
    pub rpc_source: Option<std::sync::Arc<cryo_freeze::Source>>,
    // /// current df
    // pub current_df: Option<DataFrame>,
    // /// current table
    // pub current_table: Option<String>,
}

impl EtopState {

    /// see block
    pub fn see_block(&mut self, seen_block: u32) {
        match self.latest_block {
            Some(block) => if seen_block > block {
                self.latest_block = Some(seen_block);
                if self.window.live {
                    self.set_end_block(seen_block)
                }
            },
            None => {
                self.latest_block = Some(seen_block);
                if self.window.live {
                    self.set_end_block(seen_block)
                }
            },
        }
    }

    /// query
    pub async fn query(&self, dataset: String, block_range: (u32, u32)) -> Result<DataFrame, EtopError> {
        match self.rpc_source.as_ref() {
            Some(source) => {
                todo!();
                // let query = cryo_freeze::Query {};
                // let query = std::sync::Arc::new(query);
                // cryo_freeze::collect(query, source).await.map_err(EtopError::CryoError)
            },
            None => Err(EtopError::ConnectionError("no RPC endpoint specified".to_string()))
        }
    }

    /// enable live mode
    pub fn enable_live_mode(&mut self) {
        self.window.live = true;
        if let Some(block) = self.latest_block {
            self.window.set_end_block(block);
        }
    }

    /// increment window
    pub fn increment_window(&mut self, amount: u32) {
        match (self.latest_block, self.window.end_block) {
            (Some(latest_block), Some(end_block)) => {
                if end_block + amount >= latest_block {
                    self.enable_live_mode();
                } else {
                    self.window.live = false;
                    self.window.increment_window(amount);
                }
            },
            (_, Some(end_block)) => {
                self.window.live = false;
                self.window.increment_window(end_block + amount);
            }
            _ => {},
        }
    }

    /// decrement window
    pub fn decrement_window(&mut self, amount: u32) {
        self.window.decrement_window(amount)
    }

    /// set end block
    pub fn set_end_block(&mut self, block: u32) {
        if let Some(latest_block) = self.latest_block {
            if block >= latest_block {
                self.enable_live_mode()
            } else {
                self.window.live = false;
                self.window.set_end_block(block)
            };
        } else {
            self.window.live = false;
            self.window.set_end_block(block)
        }
    }
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

impl Window {
    /// increment window
    pub fn increment_window(&mut self, amount: u32) {
        if let Some(block_number) = self.end_block {
            self.set_end_block(block_number + amount)
        }
    }

    /// decrement window
    pub fn decrement_window(&mut self, amount: u32) {
        self.live = false;
        if let Some(block_number) = self.end_block {
            if amount <= block_number {
                self.set_end_block(block_number - amount)
            }
        }
    }

    /// set end block
    pub fn set_end_block(&mut self, block: u32) {
        self.end_block = Some(block);
        match self.size {
            WindowSize::Block(size) => self.start_block = Some(block - size + 1),
        }
    }
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

// /// data source
// #[derive(Debug, Clone, Default)]
// pub enum DataSource {
//     /// rpc
//     Rpc(Option<cryo_freeze::Source>),
//     /// file
//     File(FileSource),
//     /// none
//     #[default]
//     None,
// }

// /// rpc source
// #[derive(Debug, Clone, Default)]
// pub struct RpcSource {
//     // chain_id: u64,
//     /// provider
//     pub provider: Option<std::sync::Arc<Provider::<RetryClient<Http>>>>,
// }

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

            include_header_separator_row: true,
            column_delimiter: "   ".to_string(),
            header_separator_delimiter: "───".to_string(),
            ..Default::default()
        };

        Ok(fmt.format(df)?)
    }
}
