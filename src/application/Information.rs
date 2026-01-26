use std::fs;
pub fn get_metadata_file(path: String) -> String {
    let content: String = fs::read_to_string(path).expect("Not Found File");
    return content;
}
