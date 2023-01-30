use crate::{
    detect::{self, NIFileType},
    ni_object::*,
    ni_object_data::parse,
    prelude::*,
};

pub fn extract_preset(i: &[u8]) -> std::result::Result<Vec<u8>, NIFileError> {
    match detect::filetype(i) {
        NIFileType::NIContainer => {
            info!("detected NIContainer");
            let object: NIObject = i.to_vec().try_into()?;
            match parse(object.kind, &object.data) {
                _ => todo!(),
            }
        }
        NIFileType::NIKontaktMonolith => todo!(),
        NIFileType::KoreSound => todo!(),
        NIFileType::Kontakt2 => todo!(),
        NIFileType::Unknown => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fm8() -> std::result::Result<(), NIFileError> {
        let _preset = extract_preset(include_bytes!("../test-data/fm8/000-default.nfm8"))?;
        //write!(preset);
        // assert_eq!(preset, 0);
        Ok(())
    }
}
