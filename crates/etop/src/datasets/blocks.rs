use crate::{ColumnFormatShorthand, DataSpec, DataWarehouse, EtopError};
use polars::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Blocks;

impl DataSpec for Blocks {
    fn name(&self) -> String {
        "blocks".to_string()
    }

    fn row_noun(&self) -> String {
        "blocks".to_string()
    }

    fn inputs(&self) -> Vec<String> {
        vec!["blocks".to_string()]
    }

    fn transform(&self, warehouse: &DataWarehouse) -> Result<DataFrame, EtopError> {
        warehouse
            .get_dataset("blocks")?
            .clone()
            .lazy()
            .with_column(col("base_fee_per_gas") / lit(1e9))
            .collect()
            .map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        ["block_number", "timestamp", "gas_used", "base_fee_per_gas", "author"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        vec![
            ColumnFormatShorthand::new()
                .name("block_number")
                .newline_underscores(),
            ColumnFormatShorthand::new().name("timestamp"),
            ColumnFormatShorthand::new()
                .name("gas_used")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("base_fee_per_gas")
                .display_name("base_fee")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("author"),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
