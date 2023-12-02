use crate::{DataSpec, DataWarehouse, DatasetQuery, EtopError, InputDataset, Window};
use etop_format::{ColumnFormatShorthand, DataFrameFormat};
use polars::prelude::*;
use std::collections::HashMap;

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
    /// start row of current window
    pub start_row: usize,
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

/// Scroll directions
pub enum Scroll {
    /// scroll up
    Up,
    /// scroll down
    Down,
    /// no scroll
    None,
}

// render options
impl EtopState {
    /// whether any data is available to render in current window
    pub fn can_render(&self) -> bool {
        self.warehouse.data.contains_key(self.dataset.as_str())
    }

    /// format data of current window
    pub fn format_window(
        &mut self,
        render_height: usize,
        render_width: usize,
        scroll: Scroll,
    ) -> Result<String, EtopError> {
        let dataspec = crate::load_dataspec(self.dataset.clone())?;
        let df =
            dataspec.transform(&self.warehouse, self.window.start_block, self.window.end_block)?;
        match scroll {
            Scroll::Up => {
                if self.start_row > 0 {
                    self.start_row -= 1;
                }
            }
            Scroll::Down => {
                if self.start_row <= df.height() - render_height + 1 {
                    self.start_row += 1;
                }
            }
            Scroll::None => {}
        }

        // decide which columns to use
        let column_names: Vec<String> = if let Some(columns) = dataspec.default_columns() {
            columns
        } else {
            df.schema().get_names().iter().map(|s| s.to_string()).collect()
        };

        // load column formats
        let column_formats: HashMap<String, ColumnFormatShorthand> =
            dataspec.default_column_formats().unwrap_or(HashMap::new());

        let mut columns = Vec::new();
        for column_name in column_names.into_iter() {
            if let Some(column_format) = column_formats.get(column_name.as_str()) {
                columns.push(column_format.clone())
            } else {
                let dtype: DataType = df
                    .schema()
                    .get(column_name.as_str())
                    .ok_or(EtopError::ColumnMissing(column_name.to_string()))?
                    .clone();
                columns.push(get_default_format(column_name, dtype)?);
            }
        }
        // let columns: Result<Vec<ColumnFormatShorthand>, EtopError> = columns
        //     .iter()
        //     .map(|name| {
        //         column_formats.get(name).ok_or(EtopError::ColumnMissing(name.to_string())).
        // cloned()     })
        //     .collect::<Result<Vec<_>, _>>();
        // let columns = columns?;

        let fmt = DataFrameFormat {
            column_formats: Some(columns),
            render_height: Some(render_height - 1),
            max_render_width: Some(render_width),
            start_row: Some(self.start_row),

            include_header_separator_row: true,
            column_delimiter: "   ".to_string(),
            header_separator_delimiter: "───".to_string(),
            ..Default::default()
        };
        Ok(fmt.format(df)?)
    }
}

fn get_default_format(
    column_name: String,
    dtype: DataType,
) -> Result<ColumnFormatShorthand, EtopError> {
    let fmt = match dtype {
        dtype if dtype.is_integer() => ColumnFormatShorthand::new()
            .name(column_name)
            .newline_underscores()
            .set_format(etop_format::NumberFormat::new().integer_oom().precision(1)),
        dtype if dtype.is_float() => ColumnFormatShorthand::new()
            .name(column_name)
            .newline_underscores()
            .set_format(etop_format::NumberFormat::new().float_oom().precision(1)),
        DataType::Utf8 => ColumnFormatShorthand::new()
            .name(column_name)
            .newline_underscores()
            .set_format(etop_format::StringFormat::new()),
        _ => return Err(EtopError::UnsupportedDatatype(format!("{}", dtype))),
    };
    Ok(fmt)
}
