use crate::domain::enums::ContentType;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    name: String,
    size: u64,
    is_dir: bool,
    mime_type: String,
}

impl Metadata {
    pub fn new(name: String, size: u64, is_dir: bool, mime_type: String) -> Self {
        Self {
            name: name,
            size: size,
            is_dir: is_dir,
            mime_type: mime_type,
        }
    }
}
