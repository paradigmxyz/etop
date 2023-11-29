/// ui
pub struct UI {
    /// window
    pub window: Window,
    /// other window, for comparison to main window
    pub other_window: Option<Window>,
    /// dataset being displayed
    pub dataspec: String,
    /// data source
    pub source: DataSource,
}

/// window
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

/// window dynamic
pub enum WindowDynamic {
    /// live
    Live,
    /// historic
    Historic,
}

/// window size
pub enum WindowSize {
    /// block
    Block(u64),
    // Duration(),
}

/// data source
pub enum DataSource {
    /// rpc
    Rpc(RpcSource),
    /// file
    File(FileSource),
}

/// rpc source
pub struct RpcSource {
    // chain_id: u64,
    // rpc_provider: Option<Provider<Http>>,
}

/// file source
pub struct FileSource {
    // chain_id: u64,
    // rpc_provider: None,
}
