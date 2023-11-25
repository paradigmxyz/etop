use cryo_freeze::BlockChunk;
use crate::Dataset;

struct Pipeline {
    collected: DataRange,
    window: DataRange,
    datasets: Vec<Box<dyn Dataset>>,
}

struct DataRange {
    blocks: Vec<BlockChunk>,
}
