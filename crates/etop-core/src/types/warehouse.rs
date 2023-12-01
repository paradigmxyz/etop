use crate::{EtopError, InputDataset};
use polars::prelude::*;
use std::collections::{HashMap, HashSet};

/// data warehouse
#[derive(Debug, Clone, Default)]
pub struct DataWarehouse {
    /// collected data
    pub data: HashMap<String, DataFrame>,
    /// block index
    pub block_index: HashMap<String, HashSet<u32>>,
    /// address index
    pub address_index: HashMap<String, HashSet<String>>,
}

impl DataWarehouse {
    /// get dataset
    pub fn get_dataset(&self, name: &str) -> Result<DataFrame, EtopError> {
        self.data.get(name).cloned().ok_or(EtopError::MissingData(name.into()))
    }

    /// add dataset
    pub fn add_dataset(&mut self, dataset: InputDataset, df: DataFrame) -> Result<(), EtopError> {
        let name = dataset.name();

        // update index
        match dataset {
            InputDataset::Raw(dataset) => {
                let new_blocks = df.column("block_number")?.u32()?;
                let new_blocks: Vec<u32> = new_blocks.into_iter().flatten().collect();

                // dedup
                if let Some(collected) = self.block_index.get_mut(&dataset) {
                    if new_blocks.iter().any(|block| collected.contains(block)) {
                        return Ok(())
                    }
                }

                self.update_block_index(dataset, new_blocks);
            }
            InputDataset::Derived { dataset: name, dataset_column, .. } => {
                let new_addresses = df.column(dataset_column.as_str())?.utf8()?;
                let new_addresses: Vec<String> =
                    new_addresses.into_iter().flatten().map(|x| x.to_string()).collect();

                // dedup
                if let Some(collected) = self.address_index.get_mut(&name) {
                    if new_addresses.iter().any(|address| collected.contains(address)) {
                        return Ok(())
                    }
                }

                self.update_address_index(name.to_string(), new_addresses);
            }
        };


        // update dataframes
        let new_df = match self.data.get(name.as_str()) {
            Some(old_df) => old_df.clone().vstack(&df.clone()).map_err(EtopError::PolarsError)?,
            None => df.clone(),
        };
        self.data.insert(name.clone(), new_df);

        Ok(())
    }

    fn update_block_index(&mut self, dataset: String, new_blocks: Vec<u32>) {
        match self.block_index.get_mut(&dataset) {
            Some(collected) => {
                for block in new_blocks.into_iter() {
                    collected.insert(block);
                }
            }
            None => {
                let new_set: HashSet<_> = new_blocks.into_iter().collect();
                self.block_index.insert(dataset, new_set);
            }
        }
    }

    fn update_address_index(&mut self, dataset: String, new_addresses: Vec<String>) {
        match self.address_index.get_mut(&dataset) {
            Some(collected) => {
                for address in new_addresses.into_iter() {
                    collected.insert(address);
                }
            }
            None => {
                let new_set: HashSet<_> = new_addresses.into_iter().collect();
                self.address_index.insert(dataset, new_set);
            }
        }
    }

    /// compute missing blocks
    pub fn compute_missing_blocks(&self, dataset: String, interval: (u32, u32)) -> Vec<u32> {
        let (start_block, end_block) = interval;
        let required = (start_block..=end_block).collect();
        let collected = if let Some(collected) = self.block_index.get(&dataset) {
            collected
        } else {
            return required;
        };
        required.into_iter().filter(|c| !collected.contains(c)).collect()
    }

    /// compute missing blocks
    pub fn compute_missing_addresses(&self, dataset: String, required: Vec<String>) -> Vec<String> {
        if let Some(collected) = self.address_index.get(dataset.as_str()) {
            required.into_iter().filter(|address| !collected.contains(address)).collect()
        } else {
            required
        }
    }
}
