use crate::datasets::{Blocks, Erc20TransfersByErc20, TransactionsByToAddress};
use crate::{
    load_dataspec, DataRange, DataSource, DataSpec, DataWarehouse, DatasetArgs, EtopError,
    FileSource, Window, WindowSize, UI,
};
use cryo_freeze::BlockChunk;
use polars::prelude::*;

pub(crate) fn dataset_command(args: DatasetArgs) -> Result<(), EtopError> {
    let dataset = parse_dataset(&args.dataset)?;
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

fn parse_dataset(name: &str) -> Result<Box<dyn DataSpec>, EtopError> {
    let dataset: Box<dyn DataSpec> = match name {
        "blocks" => Box::new(Blocks),
        "transactions_by_to_address" => Box::new(TransactionsByToAddress),
        "erc20_transfers_by_erc20" => Box::new(Erc20TransfersByErc20),
        _ => {
            let message = format!("could not parse dataset: {}", name);
            return Err(EtopError::ParseError(message));
        }
    };
    Ok(dataset)
}

fn load_warehouse_from_filesystem(
    dataset: &dyn DataSpec,
    data_dir: String,
) -> Result<DataWarehouse, EtopError> {
    let mut warehouse = DataWarehouse::default();

    // load each input dataset
    for input in dataset.inputs() {
        let data = load_dataset_from_file(&input, &data_dir)?;
        warehouse.data.insert(input.clone(), data.clone());

        let blocks = data.column("block_number")?.i32()?;
        let chunk = match (blocks.min(), blocks.max()) {
            (Some(start_block), Some(end_block)) => {
                BlockChunk::Range(start_block as u64, end_block as u64)
            }
            _ => {
                let message = format!("no blocks loaded for: {}", dataset.name());
                return Err(EtopError::MissingData(message));
            }
        };
        let range = vec![DataRange::Block(chunk)];
        warehouse.index.insert(input, range);
    }

    Ok(warehouse)
}

fn load_dataset_from_file(name: &str, _data_dir: &str) -> Result<DataFrame, EtopError> {
    let _dataspec = load_dataspec(name.to_string());
    todo!()
}
