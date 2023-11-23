use cryo_freeze::Datatype;


pub trait DataView {
    /// which datasets the view is constructed from
    fn inputs() -> cryo_freeze::Datatype;

    /// transform inputs into the data needed for a view
    fn transform(dfs: HashMap<Datatype, DataFrame>) -> DataFrame;

    /// get data format
    fn format() -> DataFrameFormat;
}

struct DataFrameFormat {
    column_formats: Vec<ColumnFormat>,
}

impl DataFrameFormat {
    fn n_header_lines(&self) -> usize {
        self.column_formats.map(|f| f.chars().filter(|&c| c == '\n').count())
    }
}

struct TransactionsByToAddress {
}

impl DataView for TransactionsByToAddress {
    fn inputs() -> Vec<Datatype> {
    }
}
