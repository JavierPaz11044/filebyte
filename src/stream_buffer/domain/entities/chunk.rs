use crate::metadata::domain::entities::Metadata;
use crate::stream_buffer::domain::enums::chunk_from::ChunkFrom;
pub struct Chunk {
    chunk_from: ChunkFrom,
    metadata: Metadata, //Metadata from main file, not the chunk
    buffer: Vec<u8>,
    size: usize,
    original_size: Option<usize>,
}

impl Chunk {
    pub fn new(
        chunk_from: ChunkFrom,
        metadata: Metadata,
        buffer: Vec<u8>,
        original_size: Option<usize>,
    ) -> Self {
        Self {
            chunk_from: chunk_from,
            metadata: metadata,
            size: buffer.len(),
            buffer: buffer,
            original_size: original_size,
        }
    }

    pub fn get_chunk_from(&self) -> &ChunkFrom {
        &self.chunk_from
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_original_size(&self) -> Option<usize> {
        self.original_size
    }

    pub fn get_main_metadata(&self) -> &Metadata {
        &self.metadata
    }
}
