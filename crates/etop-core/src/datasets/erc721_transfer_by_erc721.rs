use crate::{AddressQueryArgument, DataSpec, DataWarehouse, EtopError, InputDataset};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// erc721 transfers by erc721
#[derive(Clone)]
pub(crate) struct Erc721TransfersByErc721;

impl DataSpec for Erc721TransfersByErc721 {
    fn name(&self) -> String {
        "erc721_transfers_by_erc721".to_string()
    }

    fn row_noun(&self) -> String {
        "erc721s".into()
    }

    fn inputs(&self) -> Vec<InputDataset> {
        vec![
            InputDataset::Raw("erc721_transfers".into()),
            InputDataset::Derived {
                dataset: "erc721_metadata".into(),
                dataset_column: "erc721".into(),
                derived_from: "erc721_transfers".into(),
                derived_from_column: "erc721".to_string(),
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
        let erc721_transfers = warehouse.get_dataset("erc721_transfers")?;
        let erc721_metadata = warehouse.get_dataset("erc721_metadata")?;
        let erc721_transfers =
            crate::filter_by_block_number(erc721_transfers, start_block, end_block)?;
        let df = erc721_transfers
            .clone()
            .lazy()
            .group_by(["erc721"])
            .agg([
                count().alias("n_transfers"),
                col("from_address").n_unique().alias("n_senders"),
                col("to_address").n_unique().alias("n_receivers"),
                col("transaction_hash").n_unique().alias("n_txs"),
                col("token_id").n_unique().alias("n_tokens"),
                col("from_address").mode().sort(true).first().alias("most_common_sender"),
                col("to_address").mode().sort(true).first().alias("most_common_receiver"),
            ])
            .sort_by_exprs(vec![col("n_transfers"), col("erc721")], [true], true, true)
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
                erc721_metadata.lazy().select([col("erc721"), col("symbol")]),
                [col("erc721")],
                [col("erc721")],
                join_args,
            )
            .collect();
        df.map_err(EtopError::PolarsError)
    }

    fn default_columns(&self) -> Option<Vec<String>> {
        let columns = [
            "symbol",
            "n_transfers",
            "n_senders",
            "n_receivers",
            "n_txs",
            "n_tokens",
            "erc721",
            "most_common_sender",
            "most_common_receiver",
        ]
        .into_iter()
        .map(|column| column.to_string())
        .collect();

        Some(columns)
    }

    fn default_column_formats(&self) -> Option<HashMap<String, ColumnFormatShorthand>> {
        let formats = vec![
            ColumnFormatShorthand::new().name("symbol").width(9),
            ColumnFormatShorthand::new()
                .name("n_transfers")
                .display_name("n\ntrans\nfers")
                .min_width(6),
            ColumnFormatShorthand::new()
                .name("n_senders")
                .display_name("n\nsend\ners")
                .min_width(6),
            ColumnFormatShorthand::new()
                .name("n_receivers")
                .display_name("n\nrecei\nvers")
                .min_width(6),
            ColumnFormatShorthand::new().name("n_txs").min_width(6),
            ColumnFormatShorthand::new().name("n_tokens").min_width(6),
            ColumnFormatShorthand::new()
                .name("erc721")
                .display_name("erc721 address")
                .min_width(12),
            ColumnFormatShorthand::new()
                .name("most_common_sender")
                .display_name("most common sender")
                .min_width(15),
            ColumnFormatShorthand::new()
                .name("most_common_receiver")
                .display_name("most common receiver")
                .min_width(15),
        ]
        .into_iter()
        .map(|column| (column.name.clone(), column))
        .collect();

        Some(formats)
    }
}
