use std::path::PathBuf;

pub enum MetadataInput {
    PATH(PathBuf),
    BUFFER(Vec<u8>),
}
