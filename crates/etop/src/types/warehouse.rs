use crate::EtopError;
use cryo_freeze::{AddressChunk, BlockChunk, ChunkData};
use polars::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DataWarehouse {
    /// collected data
    pub data: HashMap<String, DataFrame>,

    /// data collected already
    pub index: HashMap<String, Vec<DataRange>>,
}

#[derive(Debug)]
pub enum DataRange {
    Block(BlockChunk),
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
