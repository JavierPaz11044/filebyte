use crate::entities::Metadata;

struct BufferFile {
    path: String,
    metadata: Metadata,
}

impl BufferFile {
    pub fn new(path: String, metadata: Metadata) -> Self {
        Self {
            path: path,
            metadata: metadata,
        }
    }
}
