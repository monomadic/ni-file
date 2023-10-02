use crate::{read_bytes::ReadBytesExt, Error};

pub struct GroupList;

impl GroupList {
    pub fn read<R: ReadBytesExt>(_reader: R) -> Result<Self, Error> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_voice_groups_v60() -> Result<(), Error> {
        // let file = File::open("tests/data/Objects/KontaktV42/VoiceGroups/v60/000")?;
        // assert!(VoiceGroups::read(file).is_ok());
        Ok(())
    }
}
