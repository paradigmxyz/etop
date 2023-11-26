use crate::{
    load_dataspec, load_warehouse_from_filesystem, DataSource, DatasetArgs, EtopError, FileSource,
    Window, WindowSize, UI,
};

pub(crate) fn dataset_command(args: DatasetArgs) -> Result<(), EtopError> {
    let dataset = load_dataspec(args.dataset)?;
    println!("{:?}", dataset.name());
    let warehouse = match args.data_dir {
        Some(data_dir) => load_warehouse_from_filesystem(&*dataset, data_dir)?,
        None => return Err(EtopError::ArgumentError("specify --data-dir".to_string())),
    };
    let window_size = args.window.unwrap_or(100);
    let start_block = warehouse
        .min_collected_block()
        .ok_or(EtopError::MissingData("no blocks collected".to_string()))?;
    let end_block = start_block + window_size;
    let window = Window {
        start_block,
        end_block,
        live: false,
        size: WindowSize::Block(window_size),
    };
    let _ui = UI {
        window,
        other_window: None,
        dataset,
        source: DataSource::File(FileSource {}),
    };
    println!("{:?}", warehouse);
    Ok(())
}
