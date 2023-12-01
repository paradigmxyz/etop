use crate::{DataSpec, DataWarehouse, DatasetQuery, EtopError, InputDataset, Window};
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
    /// file source (data directory)
    pub file_source: Option<String>,
    /// rpc source
    pub rpc_source: Option<std::sync::Arc<cryo_freeze::Source>>,
    //
    // cache fields
    /// current df
    pub cache_df: Option<DataFrame>,
    /// current table
    pub cache_df_render: Option<String>,
    /// messages
    pub messages: Vec<String>,
}

// state updates
impl EtopState {
    /// see block
    pub fn see_block(&mut self, seen_block: u32) {
        match self.latest_block {
            Some(block) => {
                if seen_block > block {
                    self.latest_block = Some(seen_block);
                    if self.window.live {
                        self.set_end_block(seen_block)
                    }
                }
            }
            None => {
                self.latest_block = Some(seen_block);
                if self.window.live {
                    self.set_end_block(seen_block)
                }
            }
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
        self.increment_block(amount);
    }

    /// increment block
    pub fn increment_block(&mut self, amount: u32) {
        match (self.latest_block, self.window.end_block) {
            (Some(latest_block), Some(end_block)) => {
                if end_block + amount >= latest_block {
                    self.enable_live_mode();
                } else {
                    self.window.live = false;
                    self.window.increment_block(amount);
                }
            }
            (_, Some(end_block)) => {
                self.window.live = false;
                self.window.increment_block(end_block + amount);
            }
            _ => {}
        }
    }

    /// decrement block
    pub fn decrement_window(&mut self, amount: u32) {
        self.window.decrement_window(amount)
    }

    /// decrement block
    pub fn decrement_block(&mut self, amount: u32) {
        self.window.decrement_block(amount)
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

// queries
impl EtopState {
    /// dataspec
    pub fn dataspec(&self) -> Result<Box<dyn DataSpec>, EtopError> {
        crate::load_dataspec(self.dataset.clone())
    }

    /// query
    pub async fn query(&self, query: DatasetQuery) -> Result<DataFrame, EtopError> {
        match self.rpc_source.as_ref() {
            Some(source) => query.query(source.clone()).await,
            None => Err(EtopError::ConnectionError("no RPC endpoint specified".to_string())),
        }
    }

    /// create queries for all data missing from the current view
    pub fn create_missing_queries(&self) -> Result<Vec<DatasetQuery>, EtopError> {
        let window_interval = match (self.window.start_block, self.window.end_block) {
            (Some(start_block), Some(end_block)) => (start_block, end_block),
            _ => return Ok(vec![]),
        };

        // raw inputs
        let mut queries = vec![];
        let dataspec = self.dataspec()?;
        let inputs = dataspec.inputs();
        for dataset in inputs.iter() {
            if let InputDataset::Raw(name) = dataset {
                let missing =
                    self.warehouse.compute_missing_blocks(name.to_string(), window_interval);
                if !missing.is_empty() {
                    let query = DatasetQuery::Block(dataset.clone(), missing);
                    queries.push(query)
                }
            }
        }

        // derived inputs
        for dataset in inputs.iter() {
            if let InputDataset::Derived { derived_from, derived_from_column, .. } = dataset {
                // if no addresses required, need no query
                if !self.warehouse.data.contains_key(derived_from) {
                    continue;
                }

                // compute addresses that are required
                let df = self.warehouse.get_dataset(derived_from)?;
                let required: Vec<String> = df
                    .column(derived_from_column)?
                    .unique()?
                    .utf8()?
                    .into_iter()
                    .flatten()
                    .map(|x| x.to_string())
                    .collect();

                // compute addresses that are missing
                let missing = self.warehouse.compute_missing_addresses(dataset.name(), required);
                if !missing.is_empty() {
                    queries.push(DatasetQuery::Address(dataset.clone(), missing))
                };
            }
        }

        Ok(queries)
    }
}

// render options
impl EtopState {
    /// whether any data is available to render in current window
    pub fn can_render(&self) -> bool {
        self.warehouse.data.contains_key(self.dataset.as_str())
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
