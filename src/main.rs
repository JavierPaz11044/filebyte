use crate::{
    metadata::file_metadata::ExtractMetadata, stream_buffer::services::reassembler::Reassembler,
};

mod metadata;
mod stream_buffer;
pub mod utils;

fn main() {
    println!("Hello, world!");
    stream_buffer::services::chunker::Chunker::new(ExtractMetadata::new()).chunk_file(
        "/home/lpazc/Documentos/Proyects/rust/filebyte/tests/resources/image.png".to_string(),
        |chunk| {
            println!(
                "Chunk received: size = {}, original_size = {:?}, content = {}, mimetype = {:?}",
                chunk.get_size(),
                chunk.get_original_size(),
                String::from_utf8_lossy(chunk.get_buffer()),
                chunk.get_main_metadata().get_mime_type()
            );

            Reassembler::new("/home/lpazc/Documentos/Proyects/rust/filebyte/tests/resources/reassembled_image.png".to_string())
                .write_chunk(&chunk)
                .expect("Failed to write chunk");
        },
        Some(|error| {
            eprintln!("Error occurred: {}", error);
        }),
    );
}
