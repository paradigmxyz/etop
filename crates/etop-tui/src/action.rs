use polars::prelude::*;
use etop_core::DataWarehouse;

// #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[derive(Debug, Clone)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    ToggleShowHelp,
    ScheduleIncrement,
    ScheduleDecrement,
    Increment(usize),
    Decrement(usize),
    CompleteInput(String),
    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,
    Update,
    // etop-specific
    IncrementWindow,
    DecrementWindow,
    ScheduleIncrementWindow,
    LoadDataset(String),
    NewWarehouse(DataWarehouse),
    RequestDataset(String),  // dataset name
    SetDataset(String, DataFrame),  // dataset name
    SendQuery(String, String),  // dataset name, query
    ReceiveQuery(String, String, Option<DataFrame>), //
}
