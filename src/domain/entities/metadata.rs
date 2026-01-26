use crate::domain::enums::ContentType;
struct Metadata {
    name: String,
    size: u64,
    conten_type: ContentType,
}

impl Metadata {
    pub fn new(name: String) -> Self {
        Self {
            name: name.into(),
            conten_type: ContentType::TEXT,
            size: 390,
        }
    }
}
