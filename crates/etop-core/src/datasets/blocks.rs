use crate::{DataSpec, DataWarehouse, EtopError};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// blocks
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
        vec!["blocks".to_string(), "transactions".to_string()]
    }

    fn transform(
        &self,
        warehouse: &DataWarehouse,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> Result<DataFrame, EtopError> {
        let sort = SortOptions {
            descending: true,
            nulls_last: true,
            multithreaded: true,
            maintain_order: true,
        };
        let join_args = JoinArgs {
            how: JoinType::Left,
            validation: JoinValidation::ManyToMany,
            suffix: None,
            slice: None,
        };

        let txs = warehouse
            .get_dataset("transactions")?
            .clone()
            .lazy()
            .group_by(["block_number"])
            .agg([count().alias("n_txs")])
            .collect()?;
        let blocks = warehouse
            .get_dataset("blocks")?
            .clone()
            .lazy()
            .with_column(col("base_fee_per_gas") / lit(1e9))
            .sort("block_number", sort)
            .collect()
            .map_err(EtopError::PolarsError)?;


        let blocks = crate::filter_by_block_number(blocks, start_block, end_block)?;

        blocks
            .clone()
            .lazy()
            .join(txs.lazy(), [col("block_number")], [col("block_number")], join_args)
            .collect()
            .map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        ["block_number", "timestamp", "n_txs", "gas_used", "base_fee_per_gas", "author"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        let integer_oom = etop_format::NumberFormat::new().integer_oom().precision(1);
        let float_oom = etop_format::NumberFormat::new().float_oom().precision(1);
        let timestamp_fmt = etop_format::NumberFormat::new().timestamp();

        vec![
            ColumnFormatShorthand::new().name("block_number").newline_underscores(),
            ColumnFormatShorthand::new().name("timestamp").set_format(timestamp_fmt),
            ColumnFormatShorthand::new().name("n_txs").set_format(integer_oom.clone()),
            ColumnFormatShorthand::new()
                .name("gas_used")
                .set_format(integer_oom)
                .min_width(5)
                .newline_underscores(),
            ColumnFormatShorthand::new()
                .name("base_fee_per_gas")
                .display_name("base_fee")
                .set_format(float_oom)
                .min_width(5)
                .newline_underscores(),
            ColumnFormatShorthand::new().name("author"),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
