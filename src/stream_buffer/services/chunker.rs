use std::{fs::File, io::Read, path::Path};

use crate::{
    metadata::{
        domain::{entities::Metadata, enums::MetadataInput},
        metadata_extractor::MetadataExtractor,
    },
    stream_buffer::domain::{
        constants::CHUNK_SIZE, entities::chunk::Chunk, enums::chunk_from::ChunkFrom,
    },
    utils::logger::Logger,
};

pub struct Chunker<T: MetadataExtractor> {
    console: Logger,
    metadata_extractor: T,
}

impl<T: MetadataExtractor> Chunker<T> {
    pub fn new(metadata_extractor: T) -> Self {
        let chunker = Self {
            console: Logger::new("Chunker"),
            metadata_extractor,
        };
        chunker
    }
}

impl<T: MetadataExtractor> Chunker<T> {
    pub fn chunk_file<F, E>(&self, path_file: String, mut on_chunk: F, mut on_error: Option<E>)
    where
        F: FnMut(Chunk),
        E: FnMut(String),
    {
        self.console.debug(&format!("Chunking file: {}", path_file));

        // Primero obtenés el metadata
        let path_buf = std::path::PathBuf::from(&path_file);
        let metadata_input = MetadataInput::PATH(path_buf);
        let main_metadata = self
            .metadata_extractor
            .get_metadata(metadata_input)
            .unwrap();

        self.console
            .debug(&format!("Metadata: {:?}", main_metadata));

        let file = File::open(&path_file).expect("Failed to open file");
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = [0; CHUNK_SIZE as usize];

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(bytes) => bytes,
                Err(e) => {
                    if let Some(ref mut on_error_fn) = on_error {
                        on_error_fn(e.to_string());
                    }
                    return;
                }
            };

            if bytes_read == 0 {
                break;
            }
            let chunk_data = self.get_chunk_with_metadata(
                &buffer[..bytes_read],
                &main_metadata,
                ChunkFrom::FilePath,
            );

            on_chunk(chunk_data);
        }
    }

    fn get_chunk_with_metadata(
        &self,
        chunk: &[u8],
        main_metadata: &Metadata,
        chuck_from: ChunkFrom,
    ) -> Chunk {
        Chunk::new(
            chuck_from,
            main_metadata.clone(),
            chunk.to_vec(),
            Some(main_metadata.get_size() as usize),
        )
    }
}
