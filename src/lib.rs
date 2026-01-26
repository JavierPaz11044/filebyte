mod application;
mod domain;

use application::Information;

pub fn add(left: u8, right: u8) -> String {
    Information::get_metadata_file(
        "/home/lpazc/Documentos/Proyects/rust/filebyte/src/Tast.txt".into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(255, 2);
        assert_eq!(result, "Este es un contenido generado");
    }
}
