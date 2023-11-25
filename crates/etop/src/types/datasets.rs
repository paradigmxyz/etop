use crate::{EtopError, ColumnFormat};
use polars::prelude::*;
use std::collections::HashMap;

pub trait Dataset {
    /// name of dataset
    fn name(&self) -> String;

    /// plural noun of what the rows are
    fn row_noun(&self) -> String;

    /// which datasets the view is constructed from
    fn inputs(&self) -> Vec<String>;

    /// transform inputs into the data needed for a view
    fn transform(&self, dfs: HashMap<String, DataFrame>) -> Result<DataFrame, EtopError>;

    /// default columns
    fn default_columns(&self) -> Vec<String>;

    /// default format for each column
    fn default_column_formats(&self) -> HashMap<String, ColumnFormat>;
}

