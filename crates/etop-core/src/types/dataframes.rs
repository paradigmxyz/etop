use crate::{DataSpec, DataWarehouse, EtopError, InputDataset};
use polars::prelude::*;
use std::{fs::File, path::Path};

/// read parquet
pub fn read_parquet<P: AsRef<Path>>(
    path: P,
    columns: Option<Vec<String>>,
) -> Result<DataFrame, EtopError> {
    let file = File::open(path.as_ref())
        .map_err(|_| EtopError::CouldNotOpenFile(path.as_ref().to_string_lossy().to_string()))?;

    ParquetReader::new(file)
        .with_columns(columns)
        .finish()
        .map_err(|_| EtopError::CouldNotReadFile(path.as_ref().to_string_lossy().to_string()))
}

/// load warehouse from filesystem
pub fn load_warehouse_from_filesystem(
    dataset: &dyn DataSpec,
    data_dir: String,
) -> Result<DataWarehouse, EtopError> {
    let mut warehouse = DataWarehouse::default();
    for input in dataset.inputs() {
        let data = load_dataset_from_files(&input, &data_dir)?;
        for datum in data.iter() {
            warehouse.add_dataset(input.clone(), datum.clone())?;
        }
    }
    Ok(warehouse)
}

fn load_dataset_from_files(
    dataset: &InputDataset,
    data_dir: &str,
) -> Result<Vec<DataFrame>, EtopError> {
    let name = dataset.name();
    let pattern = format!("{}/{}/*__{}__*", data_dir, name, name);
    let paths = glob::glob(&pattern)?;
    let paths: Vec<_> = paths.collect();
    // println!("for {}, {} files being loaded", name, paths.len());
    let mut dfs: Vec<DataFrame> = vec![];
    for path in paths.into_iter() {
        dfs.push(read_parquet(path?, None)?)
    }
    Ok(dfs)
}

/// filter by block number
pub fn filter_by_block_number(
    df: DataFrame,
    start_block: Option<u32>,
    end_block: Option<u32>,
) -> Result<DataFrame, EtopError> {
    let res = match (start_block, end_block) {
        (Some(start_block), Some(end_block)) => df
            .lazy()
            .filter(
                col("block_number").gt_eq(start_block).and(col("block_number").lt_eq(end_block)),
            )
            .collect()?,
        (Some(start_block), None) => {
            df.lazy().filter(col("block_number").gt_eq(start_block)).collect()?
        }
        (None, Some(end_block)) => {
            df.lazy().filter(col("block_number").lt_eq(end_block)).collect()?
        }
        (None, None) => return Ok(df),
    };
    Ok(res)
}
