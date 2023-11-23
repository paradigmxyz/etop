use crate::EtopError;
use polars::prelude::*;

#[derive(Debug)]
struct ParsedColumn {
    name: String,
    display_name: Option<String>,
    width: Option<usize>,
}

#[derive(Debug)]
struct ColumnFormat {
    name: String,
    display_name: String,
    width: usize,
}

impl ParsedColumn {
    fn to_format(&self, dtype: &DataType) -> ColumnFormat {
        let name = self.name.clone();
        let display_name = self.display_name.clone().unwrap_or(self.name.clone());
        let width = self.width.unwrap_or(get_dtype_default_width(dtype));
        ColumnFormat {
            name,
            display_name,
            width,
        }
    }
}

pub(crate) fn print_dataframe(path: String, columns: Option<Vec<String>>) -> Result<(), EtopError> {
    let columns = parse_columns(columns)?;

    // read file
    let df = read_parquet(path, &columns)?;

    // print file
    println!("{:?}", df);

    let columns: Vec<ColumnFormat> = match &columns {
        Some(columns) => columns
            .iter()
            .zip(df.schema().iter_dtypes())
            .map(|(x, dtype)| x.to_format(&dtype.clone()))
            .collect(),
        None => df
            .schema()
            .into_iter()
            .map(|(name, dtype)| ColumnFormat {
                name: name.clone().into(),
                display_name: name.clone().into(),
                width: get_dtype_default_width(&dtype),
            })
            .collect(),
    };

    let total_width = columns.iter().map(|c| c.width).sum();
    let mut header = String::with_capacity(total_width);
    for column in columns {
        header = format!(
            "{}{:>width$}",
            header,
            column.display_name,
            width = column.width
        );
    }
    println!("{}", header);

    Ok(())
}

fn get_dtype_default_width(dtype: &DataType) -> usize {
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

fn parse_columns(columns: Option<Vec<String>>) -> Result<Option<Vec<ParsedColumn>>, EtopError> {
    // syntax: COLUMN_NAME[=NEW_NAME][:WIDTH]
    match columns {
        None => Ok(None),
        Some(columns) => columns
            .into_iter()
            .map(parse_column)
            .collect::<Result<Vec<ParsedColumn>, EtopError>>()
            .map(Some),
    }
}

fn parse_column(column: String) -> Result<ParsedColumn, EtopError> {
    let parts: Vec<&str> = column.split(':').collect();
    let column_part = parts[0];

    let (name, display_name) = match column_part.split_once('=') {
        Some((name, display_name)) => (name.to_string(), Some(display_name.to_string())),
        None => (column_part.to_string(), None),
    };

    let width = if parts.len() > 1 {
        match parts[1].parse::<usize>() {
            Ok(w) => Some(w),
            Err(_) => return Err(EtopError::InvalidFormat("bad width".to_string())),
        }
    } else {
        None
    };

    Ok(ParsedColumn {
        name,
        display_name,
        width,
    })
}

fn read_parquet(path: String, columns: &Option<Vec<ParsedColumn>>) -> Result<DataFrame, EtopError> {
    let file = std::fs::File::open(path.as_str())
        .map_err(|_| EtopError::CouldNotOpenFile(path.clone()))?;
    let column_names = columns
        .as_ref()
        .map(|ccc| ccc.iter().map(|c| c.name.clone()).collect());
    ParquetReader::new(file)
        .with_columns(column_names)
        .finish()
        .map_err(|_| EtopError::CouldNotReadFile(path.clone()))
}
