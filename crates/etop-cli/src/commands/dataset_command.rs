use crate::{
    // FileSource, Window, WindowSize,
    DatasetArgs,
    EtopError,
};
use etop_core::load_dataspec;
use etop_format::{ColumnFormatShorthand, DataFrameFormat};

pub(crate) async fn dataset_command(args: DatasetArgs) -> Result<(), EtopError> {
    println!("STARTING UP");
    let mut etop_state = super::tui_command::create_etop_state(
        Some(args.dataset.clone()),
        args.block,
        args.window,
        args.rpc,
        args.data_dir,
    )
    .await?;
    println!("GATHERING QUERIES");
    let queries = etop_state.create_missing_queries()?;
    println!("MISSING QUERIES: {:?}", queries);
    println!("{:?}", etop_state.rpc_source.as_ref().unwrap().fetcher.get_block_number().await);
    for query in queries.into_iter() {
        let result = etop_state.query(query.clone()).await?;
        println!("result: {:?}", result);
        etop_state.warehouse.add_dataset(query.dataset(), result)?;
    }
    let queries = etop_state.create_missing_queries()?;
    println!("MISSING QUERIES: {:?}", queries);
    println!("{:?}", etop_state.rpc_source.as_ref().unwrap().fetcher.get_block_number().await);
    for query in queries.into_iter() {
        let result = etop_state.query(query.clone()).await?;
        println!("result: {:?}", result);
        etop_state.warehouse.add_dataset(query.dataset(), result)?;
    }

    let dataspec = load_dataspec(args.dataset)?;
    // let warehouse = match args.data_dir {
    //     Some(data_dir) => load_warehouse_from_filesystem(&*dataspec, data_dir)?,
    //     None => return Err(EtopError::ArgumentError("specify --data-dir".to_string())),
    // };

    let (render_width, render_height) = term_size::dimensions().unwrap_or((80, 20));

    let columns = dataspec.default_columns();
    let column_formats = dataspec.default_column_formats();
    let columns: Result<Vec<ColumnFormatShorthand>, EtopError> = columns
        .iter()
        .map(|name| {
            column_formats.get(name).ok_or(EtopError::ColumnMissing(name.to_string())).cloned()
        })
        .collect::<Result<Vec<_>, _>>();
    let columns = columns?;

    let df = dataspec.transform(&etop_state.warehouse, None, None)?;
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
