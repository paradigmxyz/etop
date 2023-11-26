use crate::{DataSpec, DataWarehouse, EtopError};
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

pub(crate) fn read_parquet<P: AsRef<Path>>(
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

pub(crate) fn load_warehouse_from_filesystem(
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

fn load_dataset_from_files(name: &str, data_dir: &str) -> Result<Vec<DataFrame>, EtopError> {
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
