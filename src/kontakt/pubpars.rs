#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum BProgram {
    ProgramDataV80(ProgramDataV80),
    ProgramDataVA5(ProgramDataVA5),
}

impl BProgram {
    /// BProgram::doReadPubPars
    pub fn read<R: ReadBytesExt>(mut reader: R, version: u16) -> Result<Self, Error> {
        match version {
            0x80 | 0x82 | 0x90 => {
                ProgramDataV80::read(&mut reader)?;
            }
            0x91 | 0x92 | 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 => {
                ProgramDataVA5::read(&mut reader)?;
            }
            0xA6 => {}
            0xA7 => {}
            0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAE => {}
            0xAF => {}
            _ => {}
        }

        // if version > 0xA1 {
        //     ??
        // }
    }
}
