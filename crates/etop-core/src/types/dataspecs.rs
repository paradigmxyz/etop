use crate::{datasets, DataWarehouse, EtopError};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// specification for dataset
pub trait DataSpec {
    /// name of dataset
    fn name(&self) -> String;

    /// plural noun of what the rows are
    fn row_noun(&self) -> String;

    /// which datasets the view is constructed from
    fn inputs(&self) -> Vec<String>;

    /// transform inputs into the data needed for a view
    fn transform(&self, warehouse: &DataWarehouse) -> Result<DataFrame, EtopError>;

    /// default columns
    fn default_columns(&self) -> Vec<String>;

    /// default format for each column
    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand>;
}

/// load dataspec
pub fn load_dataspec(name: String) -> Result<Box<dyn DataSpec>, EtopError> {
    match name.as_str() {
        "blocks" => Ok(Box::new(datasets::Blocks)),
        "erc20_transfers_by_erc20" => Ok(Box::new(datasets::Erc20TransfersByErc20)),
        "transactions_by_to_address" => Ok(Box::new(datasets::TransactionsByToAddress)),
        _ => Err(EtopError::UnknownData(format!("invalid dataset: {}", name))),
    }
}
