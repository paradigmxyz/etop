use crate::{DataSpec, DataWarehouse, EtopError, InputDataset};
use etop_format::{ColumnFormatShorthand, NumberFormat};
use polars::prelude::*;
use std::collections::HashMap;

/// transactions by address
#[derive(Clone)]
pub struct TransactionsByToAddress;

impl DataSpec for TransactionsByToAddress {
    fn name(&self) -> String {
        "transactions_by_to_address".into()
    }

    fn row_noun(&self) -> String {
        "to_addresses".into()
    }

    fn inputs(&self) -> Vec<InputDataset> {
        vec![InputDataset::Raw("transactions".into())]
    }

    fn transform(
        &self,
        inputs: &DataWarehouse,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> Result<DataFrame, EtopError> {
        let txs = inputs.get_dataset("transactions")?;
        let txs = crate::filter_by_block_number(txs, start_block, end_block)?;
        txs.clone()
            .lazy()
            .group_by(["to_address"])
            .agg([
                count().alias("n_txs"),
                col("value_f64").sum().alias("eth_sent") / lit(1e18),
                col("gas_price").mean().alias("mean_gas_price") / lit(1e9),
                col("gas_used").mean().alias("mean_gas_used"),
            ])
            .sort_by_exprs(vec![col("n_txs"), col("to_address")], [true, true], true, false)
            .collect()
            .map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        ["to_address", "n_txs", "eth_sent", "mean_gas_price", "mean_gas_used"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        let float_format = NumberFormat::new().si().precision(3);
        let oom_integer_format = NumberFormat::new().integer_oom().precision(0);
        let oom_float_format = NumberFormat::new().float_oom().precision(1);
        vec![
            ColumnFormatShorthand::new().name("to_address").newline_underscores(),
            ColumnFormatShorthand::new()
                .name("n_txs")
                .newline_underscores()
                .set_format(oom_integer_format.clone())
                .min_width(4),
            ColumnFormatShorthand::new()
                .name("eth_sent")
                .newline_underscores()
                .set_format(oom_float_format.clone())
                .min_width(6),
            ColumnFormatShorthand::new()
                .name("mean_gas_price")
                .newline_underscores()
                .set_format(float_format),
            ColumnFormatShorthand::new()
                .name("mean_gas_used")
                .newline_underscores()
                .set_format(oom_float_format.clone())
                .min_width(6),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
