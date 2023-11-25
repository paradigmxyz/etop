// get number of lines in header
// impl DataFrameFormat {
//     fn n_header_lines(&self) -> usize {
//         self.column_formats.map(|f| f.chars().filter(|&c| c == '\n').count())
//     }
// }

use crate::dfs::types::ColumnFormat;
use crate::EtopError;
use polars::prelude::*;

pub struct DataFrameFormat {
    pub column_formats: Option<Vec<ColumnFormat>>,
    pub column_delimiter: Option<String>,
    pub header_separator: bool,
    pub n_rows: Option<usize>,
}

pub(crate) fn print_dataframe(df: DataFrame, format: DataFrameFormat) -> Result<(), EtopError> {
    // load columns
    let columns: Vec<ColumnFormat> = match &format.column_formats {
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
    let n_rows = format.n_rows.unwrap_or_else(|| df.height().min(20));
    let widths = determine_widths(&df, &columns)?;
    let total_width = widths.iter().sum();
    let mut header = String::with_capacity(total_width);
    let column_delimiter = format.column_delimiter.unwrap_or(" ".to_string());
    for (i, (column, width)) in columns.iter().zip(widths).enumerate() {
        header.push_str(format!("{:>width$}", column.display_name, width = width).as_str());
        if i != columns.len() - 1 {
            header.push_str(column_delimiter.as_str());
        }
    };

    // let header_separator = if format.header_separator {
    // } else {
    // };

    // clip by number of rows
    let mut df = df.clone().slice(0, n_rows);

    // convert numeric fields to float64
    for (name, dtype) in df.schema().iter() {
        if dtype.is_numeric() {
            df = df.clone().with_column(df.column(name)?.to_float()?)?.clone();
        }
    };

    // print each row
    println!("{}", header);
    for r in 0..n_rows {
        let mut row = String::new();
        for (c, column_format) in columns.iter().enumerate() {
            let df = df.clone();
            let column = df.column(column_format.name.as_str())?;
            let cell = format_cell(column, column_format, r)?;
            row.push_str(cell.as_str());
            if c != columns.len() - 1 {
                row.push_str(column_delimiter.as_str());
            }
        }
        println!("{}", row);
    };

    Ok(())
}

fn format_cell(column: &Series, column_format: &ColumnFormat, r: usize) -> Result<String, EtopError> {
    match column.dtype() {
        DataType::Binary => match column.binary()?.get(r) {
            Some(binary) => Ok(column_format.binary_format()?.format(binary)?),
            None => Ok("-".into()),
        },
        DataType::Utf8 => Ok(column.str_value(r)?.to_string()),
        DataType::Float64 => match column.f64()?.get(r) {
            Some(number) => Ok(column_format.number_format()?.format(number)?),
            None => Ok("-".into()),
        },
        DataType::Boolean => match column.bool()?.get(r) {
            Some(true) => Ok("yes".to_string()),
            Some(false) => Ok("no".to_string()),
            None => Ok("-".to_string()),
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
