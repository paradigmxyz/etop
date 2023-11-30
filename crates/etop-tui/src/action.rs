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
    LiveWindow,
    PreviousWindow,
    ScheduleIncrementWindow,
    BeginBlockSubscription,
    BlockSeen(u32),
    LoadDataset(String),
    NewWarehouse(DataWarehouse),
    RequestDataset(String),  // dataset name
    SetDataset(String, DataFrame),  // dataset name
    RequestQuery(String, (u32, u32)),  // dataset name, query
    ReceiveQuery(String, (u32, u32), DataFrame), //
}
