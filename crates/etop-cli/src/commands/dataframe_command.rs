use crate::DataframeArgs;
use etop_core::EtopError;
use etop_format::{
    CellFormatShorthand, ColumnFormatShorthand, DataFrameFormat, UnknownFormat,
};

pub(crate) fn dataframe_command(args: DataframeArgs) -> Result<(), EtopError> {
    let columns = parse_columns(args.columns)?;
    let column_names: Option<Vec<String>> = columns
        .as_ref()
        .map(|cols| cols.iter().map(|c| c.name.clone()).collect());
    let df = etop_core::read_parquet(args.path, column_names)?;
    let fmt = DataFrameFormat {
        column_formats: columns,
        render_height: args.rows,
        ..Default::default()
    };
    println!("{}", fmt.format(df)?);
    Ok(())
}

pub(crate) fn parse_columns(
    columns: Option<Vec<String>>,
) -> Result<Option<Vec<ColumnFormatShorthand>>, EtopError> {
    match columns {
        None => Ok(None),
        Some(columns) => columns
            .into_iter()
            .map(parse_column)
            .collect::<Result<Vec<ColumnFormatShorthand>, EtopError>>()
            .map(Some),
    }
}

/// syntax "$COLUMN_NAME=[$DISPLAY_NAME][:$WIDTH]"
fn parse_column(column: String) -> Result<ColumnFormatShorthand, EtopError> {
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

    let format = UnknownFormat {
        min_width: width,
        max_width: width,
    };

    Ok(ColumnFormatShorthand {
        name,
        display_name,
        format: CellFormatShorthand::Unknown(format),
        ..Default::default()
    })
}
