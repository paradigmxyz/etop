use crate::{datasets, DataWarehouse, EtopError};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// Input Dataset
#[derive(Debug, Clone)]
pub enum InputDataset {
    /// Plain cryo dataset (datatype)
    Raw(String),
    /// Derived cryo dataset
    /// (datatype, dependency datatype, dependency dataype column, query argument)
    Derived {
        /// name of dataset to collect
        dataset: String,
        /// relevant column of dataset to collect
        dataset_column: String,
        /// dataset derived from
        derived_from: String,
        /// column of dataset of dataset derived from
        derived_from_column: String,
        /// argument parameter used for querying
        arg: AddressQueryArgument,
    },
}

impl InputDataset {
    /// name of dataset
    pub fn name(&self) -> String {
        match self {
            InputDataset::Raw(name) => name.to_string(),
            InputDataset::Derived { dataset, .. } => dataset.to_string(),
        }
    }
}

/// Address Query Argument
#[derive(Debug, Clone)]
pub enum AddressQueryArgument {
    /// address
    Address,
    /// contract
    Contract,
}

/// specification for dataset
pub trait DataSpec {
    /// name of dataset
    fn name(&self) -> String;

    /// plural noun of what the rows are
    fn row_noun(&self) -> String;

    /// which datasets the view is constructed from
    fn inputs(&self) -> Vec<InputDataset>;

    /// transform inputs into the data needed for a view
    fn transform(
        &self,
        warehouse: &DataWarehouse,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> Result<DataFrame, EtopError>;

    /// default columns
    fn default_columns(&self) -> Option<Vec<String>>;

    /// default format for each column
    fn default_column_formats(&self) -> Option<HashMap<String, ColumnFormatShorthand>>;
}

/// load dataspec
pub fn load_dataspec(name: String) -> Result<Box<dyn DataSpec>, EtopError> {
    match name.as_str() {
        "blocks" => Ok(Box::new(datasets::Blocks)),
        "erc20_transfers_by_erc20" => Ok(Box::new(datasets::Erc20TransfersByErc20)),
        "transactions_by_to_address" => Ok(Box::new(datasets::TransactionsByToAddress)),
        // _ => Err(EtopError::UnknownData(format!("invalid dataset: {}", name))),
        name => Ok(Box::new(datasets::CryoDataset { name: name.to_string() })),
    }
}
