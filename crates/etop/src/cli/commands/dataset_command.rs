use crate::{
    load_dataspec,
    load_warehouse_from_filesystem,
    DataFrameFormat,
    // FileSource, Window, WindowSize,
    DatasetArgs,
    EtopError,
};

pub(crate) fn dataset_command(args: DatasetArgs) -> Result<(), EtopError> {
    let dataspec = load_dataspec(args.dataset)?;
    println!("{:?}", dataspec.name());
    let warehouse = match args.data_dir {
        Some(data_dir) => load_warehouse_from_filesystem(&*dataspec, data_dir)?,
        None => return Err(EtopError::ArgumentError("specify --data-dir".to_string())),
    };
    // let window_size = args.window.unwrap_or(100);
    // let start_block = warehouse
    //     .min_collected_block()
    //     .ok_or(EtopError::MissingData("no blocks collected".to_string()))?;
    // let end_block = start_block + window_size;
    // let window = Window {
    //     start_block,
    //     end_block,
    //     live: false,
    //     size: WindowSize::Block(window_size),
    // };
    // let _ui = UI {
    //     window,
    //     other_window: None,
    //     dataspec: dataspec,
    //     source: DataSource::File(FileSource {}),
    // };
    // println!("{:?}", warehouse);

    let df = dataspec.transform(&warehouse)?;
    let fmt = DataFrameFormat {
        column_formats: None,
        column_delimiter: Some("  │  ".to_string()),
        header_separator: true,
        n_rows: Some(20),
    };
    println!("{}", fmt.format(df)?);

    Ok(())
}
