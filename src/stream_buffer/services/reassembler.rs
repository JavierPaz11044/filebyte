use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::stream_buffer::domain::entities::chunk::Chunk;

pub struct Reassembler {
    output_path: String,
}

impl Reassembler {
    pub fn new(output_path: String) -> Self {
        Self { output_path }
    }

    pub fn write_chunk(&self, chunk: &Chunk) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_path)
            .map_err(|e| e.to_string())?;

        file.write_all(&chunk.get_buffer())
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
