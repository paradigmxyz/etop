use crate::EtopError;
use cryo_freeze::{AddressChunk, BlockChunk, ChunkData};
use polars::prelude::*;
use std::collections::HashMap;

/// data warehouse
#[derive(Debug, Default)]
pub struct DataWarehouse {
    /// collected data
    pub data: HashMap<String, DataFrame>,

    /// data collected already
    pub index: HashMap<String, Vec<DataRange>>,
}

/// data range
#[derive(Debug)]
pub enum DataRange {
    /// block
    Block(BlockChunk),
    /// address
    Address(AddressChunk),
}

impl DataWarehouse {
    /// get dataset
    pub fn get_dataset(&self, name: &str) -> Result<DataFrame, EtopError> {
        self.data
            .get(name)
            .cloned()
            .ok_or(EtopError::MissingData(name.into()))
    }

    /// add dataset
    pub fn add_dataset(&mut self, name: String, data: DataFrame) -> Result<(), EtopError> {
        self.data.insert(name.clone(), data.clone());

        // load index
        let blocks = data.column("block_number")?.u32()?;
        let chunk = match (blocks.min(), blocks.max()) {
            (Some(start_block), Some(end_block)) => {
                BlockChunk::Range(start_block as u64, end_block as u64)
            }
            _ => {
                let message = format!("no blocks loaded for: {}", name);
                return Err(EtopError::MissingData(message));
            }
        };
        let range = vec![DataRange::Block(chunk)];
        self.index.insert(name, range);

        Ok(())
    }

    /// minimum collected block
    pub fn min_collected_block(&self) -> Option<u64> {
        self.index
            .values()
            .flatten()
            .filter_map(|data_range| {
                if let DataRange::Block(block_chunk) = data_range {
                    Some(block_chunk.min_value())
                } else {
                    None
                }
            })
            .min()
            .flatten()
    }
}
