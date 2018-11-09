mod double_overlapping_chunks_iterator;
mod maybe_parallel;
mod overlapping_chunks_iterator;
mod variable_chunks_iterator;

pub mod image;

#[cfg(test)]
mod test;

pub use self::double_overlapping_chunks_iterator::DoubleOverlappingChunksIterator;
pub use self::maybe_parallel::{
    process_maybe_parallel_for_each, process_maybe_parallel_map_collect,
};
pub use self::overlapping_chunks_iterator::OverlappingChunksIterator;
pub use self::variable_chunks_iterator::VariableChunksIterator;
