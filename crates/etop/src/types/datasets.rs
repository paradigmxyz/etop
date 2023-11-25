use crate::{EtopError, ColumnFormat};
use cryo_freeze::Datatype;
use polars::prelude::*;
use std::collections::HashMap;

pub trait Dataset {
    /// name of dataset
    fn name(&self) -> String;

    /// plural noun of what the rows are
    fn row_noun(&self) -> String;

    /// which datasets the view is constructed from
    fn inputs(&self) -> Vec<Datatype>;

    /// transform inputs into the data needed for a view
    fn transform(&self, dfs: HashMap<Datatype, DataFrame>) -> Result<DataFrame, EtopError>;

    /// default columns
    fn default_columns(&self) -> Vec<String>;

    /// default format for each column
    fn default_column_formats(&self) -> HashMap<String, ColumnFormat>;
}

