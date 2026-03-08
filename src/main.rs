use crate::metadata::file_metadata::ExtractMetadata;

mod metadata;
mod stream_buffer;
pub mod utils;

fn main() {
    println!("Hello, world!");
    stream_buffer::services::chunker::Chunker::new(ExtractMetadata::new()).chunk_file(
        "/home/lpazc/Documentos/Proyects/rust/filebyte/tests/resources/Information.txt".to_string(),
        |chunk| {
            println!(
                "Chunk received: size = {}, original_size = {:?}, content = {}",
                chunk.get_size(),
                chunk.get_original_size(),
                String::from_utf8_lossy(chunk.get_buffer())
            );
        },
        Some(|error| {
            eprintln!("Error occurred: {}", error);
        }),
    );
}
