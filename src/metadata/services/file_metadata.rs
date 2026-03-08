use mimetype_detector::detect_file;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    metadata::{
        domain::{entities::Metadata, enums::MetadataInput},
        services::metadata_extractor::MetadataExtractor,
    },
    utils::logger::Logger,
};

pub struct ExtractMetadata {
    console: Logger,
}

impl ExtractMetadata {
    pub fn new() -> Self {
        let metadata = Self {
            console: Logger::new("ExtractMetadata"),
        };
        metadata
    }
}

impl MetadataExtractor for ExtractMetadata {
    fn get_metadata(&self, input: MetadataInput) -> Result<Metadata, String> {
        match input {
            MetadataInput::PATH(path) => self.get_metadata_file(path),
            MetadataInput::BUFFER(_) => Err("Buffer not implemented yet".to_string()),
        }
    }
}

impl ExtractMetadata {
    fn get_metadata_file(&self, path_file: PathBuf) -> Result<Metadata, String> {
        let metadata = fs::metadata(&path_file).map_err(|e| e.to_string())?;
        let path = Path::new(&path_file);

        let file_name: String = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let mime_type: String = self.get_mime_type_file(&path_file);
        return Ok(Metadata::new(
            file_name,
            metadata.len(),
            metadata.is_dir(),
            mime_type,
        ));
    }

    fn get_mime_type_file(&self, path: &PathBuf) -> String {
        let mime_type: String = detect_file(path)
            .map(|n| n.to_string())
            .unwrap_or("Unknown".to_string());
        mime_type
    }
}
