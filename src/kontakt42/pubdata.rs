use crate::{read_bytes::ReadBytesExt, Error};

use super::zone::{ZoneV95, ZoneV98};

#[derive(Debug)]
pub enum PubData {
    ZoneV98(ZoneV98),
    ZoneV95(ZoneV95),
}

impl PubData {
    pub fn create<R: ReadBytesExt>(mut reader: R, id: u16, version: u16) -> Result<Self, Error> {
        println!("K4PL_PubData::create(0x{:x}, 0x{:x})", id, version);

        if id > 0x3e {
            panic!("id > 0x3e");
        }

        if id < 0x28 {
            panic!("id < 0x28");
        }

        match id {
            0x2c => match version {
                _ if version < 0x96 => Ok(Self::ZoneV98(ZoneV98::read(&mut reader)?)),
                _ if version < 0x99 => Ok(Self::ZoneV95(ZoneV95::read(&mut reader)?)),
                _ => panic!("unknown ZoneData"),
            },
            _ => {
                panic!("id");
            }
        }
    }
}
