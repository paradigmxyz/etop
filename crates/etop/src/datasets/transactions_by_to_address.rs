use crate::{ColumnFormat, DataSpec, DataWarehouse, EtopError};
use polars::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TransactionsByToAddress;

impl DataSpec for TransactionsByToAddress {
    fn name(&self) -> String {
        "transactions_by_to_address".into()
    }

    fn row_noun(&self) -> String {
        "to_addresses".into()
    }

    fn inputs(&self) -> Vec<String> {
        vec!["transactions".to_string()]
    }

    fn transform(&self, inputs: &DataWarehouse) -> Result<DataFrame, EtopError> {
        let txs = inputs.get_dataset("transactions")?;
        let sort = SortOptions {
            descending: true,
            nulls_last: true,
            multithreaded: true,
            maintain_order: true,
        };
        let df = txs
            .clone()
            .lazy()
            .group_by(["to_address"])
            .agg([count(), col("value_f64").sum()])
            .sort("count", sort)
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        [
            "to_address",
            "n_txs",
            "eth_sent",
            "mean_priority_fee",
            "gas_used",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormat> {
        vec![
            ColumnFormat::new()
                .name("to_address")
                .min_width(6)
                .max_width(42),
            ColumnFormat::new().name("n_txs").newline_underscores(),
            ColumnFormat::new().name("eth_sent").newline_underscores(),
            ColumnFormat::new()
                .name("mean_priority_fee")
                .display_name("mean\nprio\nfee"),
            ColumnFormat::new().name("gas_used").newline_underscores(),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
