use crate::{AddressQueryArgument, EtopError, InputDataset};
use polars::prelude::*;

/// Dataset Query
#[derive(Debug, Clone)]
pub enum DatasetQuery {
    /// Block-wise query, (dataset_name, (start_block, end_block))
    Block(InputDataset, Vec<u32>),
    /// Address-wise query, (dataset_name, address_argument, addresses)
    Address(InputDataset, Vec<String>),
}

impl DatasetQuery {
    /// dataset of query
    pub fn dataset(self) -> InputDataset {
        match self {
            DatasetQuery::Block(dataset, _) => dataset,
            DatasetQuery::Address(dataset, _) => dataset,
        }
    }
}

impl DatasetQuery {
    /// query
    pub async fn query(
        &self,
        source: std::sync::Arc<cryo_freeze::Source>,
    ) -> Result<DataFrame, EtopError> {
        let args = match self {
            DatasetQuery::Block(dataset, blocks) => {
                Self::blockwise_query(dataset.name().as_str(), blocks)
            }
            DatasetQuery::Address(dataset, addresses) => match dataset {
                InputDataset::Raw(_) => {
                    return Err(EtopError::InvalidSpecification(
                        "derived query must use derived dataset".to_string(),
                    ))
                }
                InputDataset::Derived { dataset, arg, .. } => {
                    Self::addresswise_query(dataset, arg, addresses)
                }
            },
        };
        let query = cryo_cli::parse_query(&args, source.fetcher.clone())
            .await
            .map_err(EtopError::CryoParseError)?;
        let query = std::sync::Arc::new(query);
        cryo_freeze::collect(query, source.clone()).await.map_err(EtopError::CryoError)
    }

    pub(crate) fn blockwise_query(dataset: &str, blocks: &[u32]) -> cryo_cli::Args {
        let blocks: Vec<String> =
            blocks.iter().map(|block| format!("{}:{}", block, block + 1)).collect();
        // let blocks: Vec<String> = blocks.iter().map(|block| block.to_string()).collect();
        cryo_cli::Args {
            datatype: vec![dataset.to_string()],
            blocks: Some(blocks),
            chunk_size: 1_000_000_000,
            hex: true,
            ..Default::default()
        }
    }

    pub(crate) fn addresswise_query(
        dataset: &str,
        argument: &AddressQueryArgument,
        addresses: &[String],
    ) -> cryo_cli::Args {
        let blocks = vec!["latest".to_string()];
        let args = cryo_cli::Args {
            datatype: vec![dataset.to_string()],
            blocks: Some(blocks),
            chunk_size: 1_000_000_000,
            hex: true,
            ..Default::default()
        };
        match argument {
            AddressQueryArgument::Address => {
                cryo_cli::Args { address: Some(addresses.to_vec()), ..args }
            }
            AddressQueryArgument::Contract => {
                cryo_cli::Args { contract: Some(addresses.to_vec()), ..args }
            }
        }
    }
}
