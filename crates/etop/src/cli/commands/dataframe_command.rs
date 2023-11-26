use crate::{ColumnFormat, DataFrameFormat, DataframeArgs, EtopError};

/// print dataframe command
pub(crate) fn dataframe_command(args: DataframeArgs) -> Result<(), EtopError> {
    let columns = parse_columns(args.columns)?;
    let column_names: Option<Vec<String>> = columns
        .as_ref()
        .map(|cols| cols.iter().map(|c| c.name.clone()).collect());
    let df = crate::types::read_parquet(args.path, column_names)?;
    let fmt = DataFrameFormat {
        column_formats: columns,
        column_delimiter: Some("  │  ".to_string()),
        header_separator: true,
        n_rows: args.rows,
    };
    println!("{}", fmt.format(df)?);
    Ok(())
}

pub(crate) fn parse_columns(
    columns: Option<Vec<String>>,
) -> Result<Option<Vec<ColumnFormat>>, EtopError> {
    match columns {
        None => Ok(None),
        Some(columns) => columns
            .into_iter()
            .map(parse_column)
            .collect::<Result<Vec<ColumnFormat>, EtopError>>()
            .map(Some),
    }
}

fn parse_column(column: String) -> Result<ColumnFormat, EtopError> {
    let parts: Vec<&str> = column.split(':').collect();
    let column_part = parts[0];

    let (name, display_name) = match column_part.split_once('=') {
        Some((name, display_name)) => (name.to_string(), display_name.to_string()),
        None => (column_part.to_string(), column_part.to_string()),
    };

    let width = if parts.len() > 1 {
        match parts[1].parse::<usize>() {
            Ok(w) => Some(w),
            Err(_) => return Err(EtopError::InvalidFormat("bad width".to_string())),
        }
    } else {
        None
    };

    Ok(ColumnFormat {
        name,
        display_name,
        min_width: width,
        max_width: width,
        format: None,
    })
}
