use crate::{ColumnFormat, DataSpec, DataWarehouse, EtopError};
use polars::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Erc20TransfersByErc20;

impl DataSpec for Erc20TransfersByErc20 {
    fn name(&self) -> String {
        "erc20_transfers_by_erc20".to_string()
    }

    fn row_noun(&self) -> String {
        "erc20s".into()
    }

    fn inputs(&self) -> Vec<String> {
        vec!["erc20_transfers".to_string()]
    }

    fn transform(&self, warehouse: &DataWarehouse) -> Result<DataFrame, EtopError> {
        let erc20_transfers = warehouse.get_dataset("erc20_transfers")?;
        let sort = SortOptions {
            descending: true,
            nulls_last: true,
            multithreaded: true,
            maintain_order: true,
        };
        let df = erc20_transfers
            .clone()
            .lazy()
            .group_by(["erc20"])
            .agg([
                count().alias("n_transfers"),
                col("from_address").n_unique().alias("n_senders"),
                col("to_address").n_unique().alias("n_receivers"),
            ])
            .sort("n_transfers", sort)
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        ["erc20", "n_transfers", "n_senders", "n_receivers"]
            .into_iter()
            .map(|column| column.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormat> {
        vec![
            ColumnFormat::new()
                .name("erc20")
                .display_name("erc20")
                .width(10),
            ColumnFormat::new().name("n_transfers").width(10),
            ColumnFormat::new().name("n_senders").width(10),
            ColumnFormat::new().name("n_receivers").width(10),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
