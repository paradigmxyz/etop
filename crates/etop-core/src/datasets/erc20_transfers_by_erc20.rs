use crate::{AddressQueryArgument, DataSpec, DataWarehouse, EtopError, InputDataset};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// erc20 transfers by erc20
#[derive(Clone)]
pub struct Erc20TransfersByErc20;

impl DataSpec for Erc20TransfersByErc20 {
    fn name(&self) -> String {
        "erc20_transfers_by_erc20".to_string()
    }

    fn row_noun(&self) -> String {
        "erc20s".into()
    }

    fn inputs(&self) -> Vec<InputDataset> {
        vec![
            InputDataset::Raw("erc20_transfers".into()),
            InputDataset::Derived {
                dataset: "erc20_metadata".into(),
                dataset_column: "erc20".into(),
                derived_from: "erc20_transfers".into(),
                derived_from_column: "erc20".to_string(),
                arg: AddressQueryArgument::Contract,
            },
        ]
    }

    fn transform(
        &self,
        warehouse: &DataWarehouse,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> Result<DataFrame, EtopError> {
        let erc20_transfers = warehouse.get_dataset("erc20_transfers")?;
        let erc20_metadata = warehouse.get_dataset("erc20_metadata")?;
        let erc20_transfers =
            crate::filter_by_block_number(erc20_transfers, start_block, end_block)?;
        let df = erc20_transfers
            .clone()
            .lazy()
            .group_by(["erc20"])
            .agg([
                count().alias("n_transfers"),
                col("from_address").n_unique().alias("n_senders"),
                col("to_address").n_unique().alias("n_receivers"),
                col("transaction_hash").n_unique().alias("n_txs"),
                col("value_f64").sum().alias("volume"),
                col("from_address").mode().sort(true).first().alias("most_common_sender"),
                col("to_address").mode().sort(true).first().alias("most_common_receiver"),
            ])
            .sort_by_exprs(vec![col("n_transfers"), col("erc20")], [true], true, true)
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
                erc20_metadata.lazy().select([col("erc20"), col("symbol"), col("decimals")]),
                [col("erc20")],
                [col("erc20")],
                join_args,
            )
            .with_column(col("volume") / lit(10).pow(col("decimals")))
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Vec<String> {
        [
            "symbol",
            "n_transfers",
            "n_senders",
            "n_receivers",
            "n_txs",
            "volume",
            "erc20",
            "most_common_sender",
            "most_common_receiver",
        ]
        .into_iter()
        .map(|column| column.to_string())
        .collect()
    }

    fn default_column_formats(&self) -> HashMap<String, ColumnFormatShorthand> {
        let oom_float_format = etop_format::NumberFormat::new().float_oom().precision(1);
        vec![
            ColumnFormatShorthand::new().name("symbol").width(9),
            ColumnFormatShorthand::new().name("n_transfers").display_name("n\ntrans\nfers"),
            ColumnFormatShorthand::new().name("n_senders").display_name("n\nsend\ners"),
            ColumnFormatShorthand::new().name("n_receivers").display_name("n\nrecei\nvers"),
            ColumnFormatShorthand::new().name("n_txs"),
            ColumnFormatShorthand::new().name("volume").set_format(oom_float_format).min_width(6),
            ColumnFormatShorthand::new().name("erc20").display_name("erc20 address"),
            ColumnFormatShorthand::new()
                .name("most_common_sender")
                .display_name("most common sender"),
            ColumnFormatShorthand::new()
                .name("most_common_receiver")
                .display_name("most common receiver"),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect()
    }
}
