use crate::EtopError;
use polars::prelude::*;

pub(crate) fn read_parquet(
    path: String,
    columns: Option<Vec<String>>,
) -> Result<DataFrame, EtopError> {
    let file = std::fs::File::open(path.as_str())
        .map_err(|_| EtopError::CouldNotOpenFile(path.clone()))?;
    ParquetReader::new(file)
        .with_columns(columns)
        .finish()
        .map_err(|_| EtopError::CouldNotReadFile(path.clone()))
}
