use crate::DataSpec;

pub struct UI {
    /// window
    pub window: Window,
    /// other window, for comparison to main window
    pub other_window: Option<Window>,
    /// dataset being displayed
    pub dataset: Box<dyn DataSpec>,
    /// data source
    pub source: DataSource,
}

pub struct Window {
    /// start block of window
    pub start_block: u64,
    /// end block of window
    pub end_block: u64,
    /// whether window is historic or updates with live data
    pub live: bool,
    /// size of window, in blocks or in time
    pub size: WindowSize,
}

pub enum WindowDynamic {
    Live,
    Historic,
}

pub enum WindowSize {
    Block(u64),
    // Duration(),
}

pub enum DataSource {
    Rpc(RpcSource),
    File(FileSource),
}

pub struct RpcSource {
    // chain_id: u64,
    // rpc_provider: Option<Provider<Http>>,
}

pub struct FileSource {
    // chain_id: u64,
    // rpc_provider: None,
}
