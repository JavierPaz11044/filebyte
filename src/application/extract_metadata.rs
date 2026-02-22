use crate::{entities::Metadata, enums::ContentType, utils::logger::Logger};
use mimetype_detector::{MimeType, detect_file, mime_type};
use std::{fs, path::Path};

pub struct ExtractMetadata {
    path: String,
    console: Logger,
}

impl ExtractMetadata {
    pub fn new(path: String) -> Self {
        let metadata = Self {
            path: path,
            console: Logger::new("ExtractMetadata"),
        };
        //   metadata.builder();
        metadata
    }

    pub fn get_metadata_file(&self, path: String) -> Metadata {
        let metadata = fs::metadata(&path).expect("Path not found");
        let path = Path::new(&path);

        let file_name: String = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let mime_type = self.get_mime_type_file();
        return Metadata::new(file_name, metadata.len(), metadata.is_dir(), mime_type);
    }

    fn get_mime_type_file(&self) -> String {
        let mime_type = detect_file(&self.path)
            .map(|n| n.to_string())
            .unwrap_or("Unknown".to_string());
        mime_type
    }
}
