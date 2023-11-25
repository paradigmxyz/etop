use crate::{ColumnFormat, Dataset, EtopError};
use polars::prelude::*;
use std::collections::HashMap;

struct TransactionsByToAddress;

impl Dataset for TransactionsByToAddress {
    fn name(&self) -> String {
        "transactions_by_to_address".into()
    }

    fn row_noun(&self) -> String {
        "to_addresses".into()
    }

    fn inputs(&self) -> Vec<String> {
        vec!["transactions".to_string()]
    }

    fn transform(&self, inputs: HashMap<String, DataFrame>) -> Result<DataFrame, EtopError> {
        if let Some(txs) = inputs.get("transactions") {
            let df = txs
                .clone()
                .lazy()
                .group_by(["to_address"])
                .agg([count(), col("value_f64").sum()])
                .collect();
            df.map_err(EtopError::PolarsError)
        } else {
            Err(EtopError::TransformError("df missing for txs".to_string()))
        }
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
