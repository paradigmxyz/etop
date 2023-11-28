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
        warehouse.get_dataset("blocks")
    }

    fn default_columns(&self) -> Vec<String> {
        ["block_number", "gas_used", "base_fee"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        vec![
            ColumnFormatShorthand::new()
                .name("block_number")
                .min_width(12)
                .max_width(12)
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("gas_used")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("base_fee")
                .newline_underscores(),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
