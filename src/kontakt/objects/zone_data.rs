use std::io::Cursor;

use crate::{kontakt::structured_object::StructuredObject, read_bytes::ReadBytesExt, Error};

#[derive(Debug)]
pub struct Zone(pub StructuredObject);

/// Type:           StructuredObject
/// Kontakt 7:      BZone, BProgram::readZones()
/// KontaktIO:      K4PL_Zone<K4PO::K4PL_ZoneDataV95>
#[derive(Debug)]
pub struct ZoneParams {
    pub sample_start: i32,
    pub sample_end: i32,
    pub sample_start_mod_range: i32,
    pub low_velocity: i16,
    pub high_velocity: i16,
    pub low_key: i16,
    pub high_key: i16,
    pub fade_low_velocity: i16,
    pub fade_high_velocity: i16,
    pub fade_low_key: i16,
    pub fade_high_key: i16,
    pub root_key: i16,
    pub zone_volume: f32,
    pub zone_pan: f32,
    pub zone_tune: f32,
    /// The index of the file in the filetable.
    pub filename_id: i32,
    pub sample_data_type: i32,
    pub sample_rate: i32,
    pub num_channels: u8,
    pub num_frames: i32,
    pub reserved1: i32,
    pub reserved2: Option<i32>,
    pub root_note: i32,
    pub tuning: f32,
    pub reserved3: bool,
    pub reserved4: i32,
    // LoopArray 0x39
    // QuickBrowseData 0x4e
    // PrivateRawObject 0x35
}

impl Zone {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }

    pub fn params(&self) -> Result<ZoneParams, Error> {
        let mut reader = Cursor::new(&self.0.public_data);

        for chunk in &self.0.children {
            println!("{:?} {:x}", chunk.into_object()?, chunk.id);
        }

        Ok(ZoneParams {
            sample_start: reader.read_i32_le()?,
            sample_end: reader.read_i32_le()?,
            sample_start_mod_range: reader.read_i32_le()?,
            low_velocity: reader.read_i16_le()?,
            high_velocity: reader.read_i16_le()?,
            low_key: reader.read_i16_le()?,
            high_key: reader.read_i16_le()?,
            fade_low_velocity: reader.read_i16_le()?,
            fade_high_velocity: reader.read_i16_le()?,
            fade_low_key: reader.read_i16_le()?,
            fade_high_key: reader.read_i16_le()?,
            root_key: reader.read_i16_le()?,
            zone_volume: reader.read_f32_le()?,
            zone_pan: reader.read_f32_le()?,
            zone_tune: reader.read_f32_le()?,
            filename_id: reader.read_i32_le()?,
            sample_data_type: reader.read_i32_le()?,
            sample_rate: reader.read_i32_le()?,
            num_channels: reader.read_u8()?,
            num_frames: reader.read_i32_le()?,
            reserved1: reader.read_i32_le()?,
            reserved2: {
                match self.0.version {
                    _ if self.0.version < 0x96 => Some(reader.read_i32_le()?),
                    _ if self.0.version < 0x99 => None,
                    _ => None,
                }
            },
            root_note: reader.read_i32_le()?,
            tuning: reader.read_f32_le()?,
            reserved3: reader.read_bool()?,
            reserved4: reader.read_i32_le()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
    use crate::Error;

    #[test]
    fn test_zone_data_v9a_000() -> Result<(), Error> {
        let file =
            File::open("tests/data/Objects/Kontakt/ZoneData/ZoneDataV9A/ZoneDataV9A-000.kon")?;
        let zone = Zone::read(file)?;
        assert_eq!(zone.0.version, 0x9A);
        zone.params()?;
        Ok(())
    }
}
