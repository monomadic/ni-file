use crate::{
    kontakt::{
        objects::{Bank, FNTableImpl},
        Chunk,
    },
    read_bytes::ReadBytesExt,
    Error,
};

#[derive(Debug)]
pub struct KontaktMulti {
    pub bank: Bank,
    pub filetable: FNTableImpl,
}

impl KontaktMulti {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let bank: Bank = Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;
        dbg!(&bank);

        let filetable: FNTableImpl =
            Chunk::read(&mut reader).and_then(|chunk| (&chunk).try_into())?;
        dbg!(&filetable);

        Ok(Self { bank, filetable })
    }
}
