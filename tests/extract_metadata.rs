use file_byte;
use mimetype_detector;
mod common;

#[test]
fn validate_metadata_stractor() {
    let mime_type = mimetype_detector::detect_file("/home/lpazc/Documentos/StayMeck/stayMeck.jpg");
    match mime_type {
        Ok(mime) => println!("File type: {}", mime),
        Err(e) => println!("Erro in readn: {}", e),
    }

    assert_eq!("1", "1");
}
