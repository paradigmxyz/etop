use crate::{ColumnFormatShorthand, DataSpec, DataWarehouse, EtopError};
use polars::prelude::*;
use std::collections::HashMap;
use toolstr::NumberFormat;

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
            .agg([
                count().alias("n_txs"),
                col("value_f64").sum().alias("eth_sent") / lit(1e18),
                col("gas_price").mean().alias("mean_gas_price") / lit(1e9),
                col("gas_used").mean().alias("mean_gas_used"),
            ])
            .sort("n_txs", sort)
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        [
            "to_address",
            "n_txs",
            "eth_sent",
            "mean_gas_price",
            "mean_gas_used",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        let float_format = NumberFormat::new().si().precision(3);
        let oom_integer_format = NumberFormat::new().integer_oom().precision(0);
        let oom_float_format = NumberFormat::new().float_oom().precision(1);
        vec![
            ColumnFormatShorthand::new()
                .name("to_address")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("n_txs")
                .newline_underscores()
                .set_format(oom_integer_format.clone()),
            ColumnFormatShorthand::new()
                .name("eth_sent")
                .newline_underscores()
                .set_format(oom_float_format.clone()),
            ColumnFormatShorthand::new()
                .name("mean_gas_price")
                .newline_underscores()
                .set_format(float_format),
            ColumnFormatShorthand::new()
                .name("mean_gas_used")
                .newline_underscores()
                .set_format(oom_float_format.clone()),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
