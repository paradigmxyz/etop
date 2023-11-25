use crate::cli::args::DataframeArgs;
use crate::dfs;
use crate::ColumnFormat;
use crate::EtopError;

/// print dataframe command
pub(crate) fn dataframe_command(args: DataframeArgs) -> Result<(), EtopError> {
    println!("path: {}", args.path);
    println!("format: {:?}", args.format);
    println!();
    let columns = parse_columns(args.columns)?;
    // let columns = columns.map(|cols| cols.into_iter().map(|c| c.to_format()).collect()).collect();
    // let column_names = columns
    //     .as_ref()
    //     .map(|ccc| ccc.iter().map(|c| c.name.clone()).collect());
    dfs::print_dataframe(args.path, columns, args.rows).unwrap();
    Ok(())
}

pub(crate) fn parse_columns(columns: Option<Vec<String>>) -> Result<Option<Vec<ColumnFormat>>, EtopError> {
    // syntax: COLUMN_NAME[=NEW_NAME][:WIDTH]
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
