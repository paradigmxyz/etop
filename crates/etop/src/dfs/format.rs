use super::io;
use crate::dfs::types::ColumnFormat;
use crate::EtopError;
use polars::prelude::*;
// use polars_core::prelude::*;

pub(crate) fn print_dataframe(
    path: String,
    columns: Option<Vec<ColumnFormat>>,
    n_rows: Option<usize>,
) -> Result<(), EtopError> {

    // read file
    let column_names: Option<Vec<String>> = columns
        .as_ref()
        .map(|cols| cols.iter().map(|c| c.name.clone()).collect());
    let mut df = io::read_parquet(path, column_names)?;

    // load columns
    let columns: Vec<ColumnFormat> = match &columns {
        Some(c) => c.to_owned(),
        None => df
            .schema()
            .into_iter()
            .map(|(name, _)| ColumnFormat {
                name: name.clone().into(),
                display_name: name.clone().into(),
                min_width: None,
                max_width: None,
                format: None,
            })
            .collect(),
    };

    // build header row
    let n_rows = n_rows.unwrap_or_else(|| df.height().min(20));
    let widths = determine_widths(&df, &columns)?;
    let total_width = widths.iter().sum();
    let mut header = String::with_capacity(total_width);
    for (column, width) in columns.iter().zip(widths) {
        header = format!("{}{:>width$}", header, column.display_name, width = width);
    }

    // print file
    println!("{:?}", df);
    println!();
    println!("{}", header);

    // clip by number of rows
    df = df.slice(0, n_rows);

    // convert numeric fields to float64
    for (name, dtype) in df.schema().iter() {
        if dtype.is_numeric() {
            df = df.clone().with_column(df.column(name)?.to_float()?)?.clone();
        }
    };

    // print each row
    for r in 0..n_rows {
        let mut row = String::new();
        for column_format in columns.iter() {
            let df = df.clone();
            let column = df.column(column_format.name.as_str())?;
            let cell = format_cell(column, column_format, r)?;
            row = format!("{} {}", row, cell);
        }
        println!("{}", row);
    };

    Ok(())
}

// use to_float() to convert all numeric types to f64
fn format_cell(column: &Series, column_format: &ColumnFormat, r: usize) -> Result<String, EtopError> {
    match column.dtype() {
        DataType::Binary => Ok("BINARY".to_string()),
            // match column.binary()?.get(r) {
            // Some(number) => Ok(column_format.number_format()?.format(number)?),
            // None => Ok("-".into()),
        // },
        DataType::Utf8 => Ok(column.str_value(r)?.to_string()),
        DataType::Float64 => match column.f64()?.get(r) {
            Some(number) => Ok(column_format.number_format()?.format(number)?),
            None => Ok("-".into()),
        },
        dtype => Err(EtopError::UnsupportedDatatype(dtype.to_string())),
    }
}

pub(crate) fn determine_widths(
    df: &DataFrame,
    columns: &Vec<ColumnFormat>,
) -> Result<Vec<usize>, EtopError> {
    let mut widths = Vec::with_capacity(columns.len());
    for column in columns.iter() {
        match column.min_width {
            Some(min_width) => widths.push(min_width),
            None => match df.schema().get(column.name.as_str()) {
                Some(dtype) => widths.push(get_dtype_default_width(dtype)),
                None => return Err(EtopError::ColumnMissing(column.name.to_string())),
            },
        }
    }
    Ok(widths)
}

pub(crate) fn get_dtype_default_width(dtype: &DataType) -> usize {
    match dtype {
        DataType::Boolean => 12,
        DataType::UInt8 => 12,
        DataType::UInt16 => 12,
        DataType::UInt32 => 12,
        DataType::UInt64 => 12,
        DataType::Int8 => 12,
        DataType::Int16 => 12,
        DataType::Int32 => 12,
        DataType::Int64 => 12,
        DataType::Float32 => 12,
        DataType::Float64 => 12,
        // DataType::Decimal(_precision, _scale) => 12,
        DataType::Utf8 => 12,
        DataType::Binary => 12,
        DataType::Date => 12,
        DataType::Datetime(_, _) => 12,
        DataType::Duration(_unit) => 12,
        DataType::Time => 12,
        // DataType::Array(_datatype, _size) => 12,
        DataType::List(_datatype) => 12,
        // DataType::Object(_) => 12,
        DataType::Null => 12,
        // DataType::Categorical(_) => 12,
        DataType::Struct(_fields) => 12,
        DataType::Unknown => 12,
    }
}
