use crate::metadata::domain::{entities::Metadata, enums::MetadataInput};

pub trait MetadataExtractor {
    fn get_metadata(&self, input: MetadataInput) -> Result<Metadata, String>;
}
