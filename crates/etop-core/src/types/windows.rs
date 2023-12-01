/// window
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// start block of window
    pub start_block: Option<u32>,
    /// end block of window
    pub end_block: Option<u32>,
    /// whether window is historic or updates with live data
    pub live: bool,
    /// size of window, in blocks or in time
    pub size: WindowSize,
}

impl Window {
    /// increment window
    pub fn increment_window(&mut self, amount: u32) {
        let WindowSize::Block(size) = self.size.clone();
        self.increment_block(amount * size);
    }

    /// increment block
    pub fn increment_block(&mut self, amount: u32) {
        if let Some(block_number) = self.end_block {
            self.set_end_block(block_number + amount)
        }
    }

    /// decrement window
    pub fn decrement_window(&mut self, amount: u32) {
        self.live = false;
        let WindowSize::Block(size) = self.size.clone();
        self.decrement_block(amount * size);
    }

    /// decrement block
    pub fn decrement_block(&mut self, amount: u32) {
        self.live = false;
        if let Some(block_number) = self.end_block {
            if amount <= block_number {
                self.set_end_block(block_number - amount)
            }
        }
    }

    /// set end block
    pub fn set_end_block(&mut self, block: u32) {
        self.end_block = Some(block);
        match self.size {
            WindowSize::Block(size) => self.start_block = Some(block - size + 1),
        }
    }
}

/// window size
#[derive(Debug, Clone)]
pub enum WindowSize {
    /// block
    Block(u32),
    // Duration(),
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        WindowSize::Block(1)
    }
}
