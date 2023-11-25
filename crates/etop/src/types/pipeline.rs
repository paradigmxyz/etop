use cryo_freeze::BlockChunk;
use crate::Dataset;

pub struct Pipeline {
    collected: DataRange,
    window: DataRange,
    datasets: Vec<Box<dyn Dataset>>,
}

pub struct DataRange {
    blocks: Vec<BlockChunk>,
}
