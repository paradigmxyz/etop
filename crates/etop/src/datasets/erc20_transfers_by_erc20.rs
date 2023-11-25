use crate::{EtopError, ColumnFormat, Dataset};
use polars::prelude::*;
use std::collections::HashMap;

struct Erc20TransfersByErc20s;

impl Dataset for Erc20TransfersByErc20s {
    /// name of dataset
    fn name(&self) -> String {
        "erc20_transfers_by_erc20".to_string()
    }

    /// plural noun of what the rows are
    fn row_noun(&self) -> String {
        "erc20s".into()
    }

    /// which datasets the view is constructed from
    fn inputs(&self) -> Vec<String> {
        vec!["erc20_transfers".to_string()]
    }

    /// transform inputs into the data needed for a view
    fn transform(&self, _dfs: HashMap<String, DataFrame>) -> Result<DataFrame, EtopError> {
        todo!();
    }

    /// default columns
    fn default_columns(&self) -> Vec<String> {
        vec![
            "erc20",
            "n_transfers",
            "n_senders",
            "n_receivers",
        ]
        .into_iter()
        .map(|column| column.to_string())
        .collect()
    }

    /// default format for each column
    fn default_column_formats(&self) -> HashMap<String, ColumnFormat> {
        vec![
            ColumnFormat::new().name("erc20").display_name("erc20").width(10),
            ColumnFormat::new().name("n_transfers").width(10),
            ColumnFormat::new().name("n_senders").width(10),
            ColumnFormat::new().name("n_receivers").width(10),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}

