use crate::{ColumnFormatShorthand, DataSpec, DataWarehouse, EtopError};
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
        vec!["erc20_transfers".to_string(), "erc20_metadata".to_string()]
    }

    fn transform(&self, warehouse: &DataWarehouse) -> Result<DataFrame, EtopError> {
        let erc20_transfers = warehouse.get_dataset("erc20_transfers")?;
        let erc20_metadata = warehouse.get_dataset("erc20_metadata")?;
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
        let df = df.map_err(EtopError::PolarsError)?;
        let join_args = JoinArgs {
            how: JoinType::Left,
            validation: JoinValidation::ManyToMany,
            suffix: None,
            slice: None,
        };
        let df = df
            .clone()
            .lazy()
            .join(
                erc20_metadata.lazy().select([col("erc20"), col("symbol")]),
                [col("erc20")],
                [col("erc20")],
                join_args,
            )
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        ["symbol", "n_transfers", "n_senders", "n_receivers", "erc20"]
            .into_iter()
            .map(|column| column.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        vec![
            ColumnFormatShorthand::new().name("symbol").max_width(9),
            ColumnFormatShorthand::new()
                .name("n_transfers")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("n_senders")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("n_receivers")
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("erc20")
                .display_name("address"),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
