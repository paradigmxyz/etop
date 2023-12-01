use crate::{DataSpec, DataWarehouse, EtopError, InputDataset};
use etop_format::ColumnFormatShorthand;
use polars::prelude::*;
use std::collections::HashMap;

/// cryo dataset
#[derive(Clone)]
pub struct CryoDataset {
    /// name
    pub name: String,
}

impl DataSpec for CryoDataset {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn row_noun(&self) -> String {
        self.name.to_string()
    }

    fn inputs(&self) -> Vec<InputDataset> {
        vec![InputDataset::Raw(self.name.to_string())]
    }

    fn transform(
        &self,
        warehouse: &DataWarehouse,
        start_block: Option<u32>,
        end_block: Option<u32>,
    ) -> Result<DataFrame, EtopError> {
        let mut df = warehouse.get_dataset(self.name.as_str())?;
        df = crate::filter_by_block_number(df, start_block, end_block)?;

        for (name, dtype) in df.schema().iter() {
            if !dtype.is_integer() & !dtype.is_float() {
                match dtype {
                    DataType::Utf8 => {}
                    _ => {
                        df = df
                            .lazy()
                            .drop_columns(vec![name])
                            // .with_column(col(name).cast(DataType::Utf8))
                            .collect()
                            .map_err(EtopError::PolarsError)?;
                    }
                }
            }
        }

        Ok(df)
    }

    fn default_columns(&self) -> Option<Vec<String>> {
        None
    }

    fn default_column_formats(&self) -> Option<HashMap<String, ColumnFormatShorthand>> {
        None
    }
}
