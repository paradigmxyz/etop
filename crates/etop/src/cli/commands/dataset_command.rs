use crate::{
    load_dataspec,
    load_warehouse_from_filesystem,
    ColumnFormatShorthand,
    DataFrameFormat,
    // FileSource, Window, WindowSize,
    DatasetArgs,
    EtopError,
};

pub(crate) fn dataset_command(args: DatasetArgs) -> Result<(), EtopError> {
    let dataspec = load_dataspec(args.dataset)?;
    let warehouse = match args.data_dir {
        Some(data_dir) => load_warehouse_from_filesystem(&*dataspec, data_dir)?,
        None => return Err(EtopError::ArgumentError("specify --data-dir".to_string())),
    };

    let (render_width, render_height) = term_size::dimensions().unwrap_or((80, 20));

    let columns = dataspec.default_columns();
    let column_formats = dataspec.default_column_formats();
    let columns: Result<Vec<ColumnFormatShorthand>, EtopError> = columns
        .iter()
        .map(|name| {
            column_formats
                .get(name)
                .ok_or(EtopError::ColumnMissing(name.to_string()))
                .cloned()
        })
        .collect::<Result<Vec<_>, _>>();
    let columns = columns?;

    let df = dataspec.transform(&warehouse)?;
    let fmt = DataFrameFormat {
        column_formats: Some(columns),
        render_height: Some(render_height - 1),
        max_render_width: Some(render_width),
        ..Default::default()
    };
    println!("{}", fmt.format(df)?);

    Ok(())
}

// fn create_ui() {
//     // let window_size = args.window.unwrap_or(100);
//     // let start_block = warehouse
//     //     .min_collected_block()
//     //     .ok_or(EtopError::MissingData("no blocks collected".to_string()))?;
//     // let end_block = start_block + window_size;
//     // let window = Window {
//     //     start_block,
//     //     end_block,
//     //     live: false,
//     //     size: WindowSize::Block(window_size),
//     // };
//     // let _ui = UI {
//     //     window,
//     //     other_window: None,
//     //     dataspec: dataspec,
//     //     source: DataSource::File(FileSource {}),
//     // };
//     // println!("{:?}", warehouse);
// }
