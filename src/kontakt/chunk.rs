use std::io::Cursor;

use crate::{read_bytes::ReadBytesExt, Error};

use super::{
    objects::{Bank, FNTableImpl, GroupList, Program, VoiceGroups},
    structured_object::StructuredObject,
};

#[derive(Debug)]
pub struct Chunk {
    pub id: u16,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()? as usize;
        let data = reader.read_bytes(length)?;
        Ok(Self { id, data })
    }

    pub fn into_object(&self) -> Result<KontaktObject, Error> {
        Ok(KontaktObject::try_from(self)?)
    }
}

impl std::convert::TryFrom<&Chunk> for StructuredObject {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        let cursor = Cursor::new(&chunk.data);
        Ok(StructuredObject::read(cursor)?)
    }
}

#[derive(Debug)]
pub enum KontaktObject {
    BParModBase,
    BAutomationObject,
    OutputPartition,
    BBank(Bank),
    BGroup,
    BLoop,
    BParScript,
    BParEnv,
    BParLFO,
    BParArp,
    BParEnvF,
    BParGlide,
    BParExternalMod,
    BParInternalMod,
    BParSrcMode,
    BParStartCriteria,
    BParFXDelay,
    BParFXChorus,
    BParFXFlanger,
    BParFXGainer,
    BParFXPhaser,
    BParFXReverb,
    BParFXIRC,
    BParFXSendLevel,
    BParFXSendLevels,
    BParFXFilter,
    BParFXCompressor,
    BParFXInverter,
    BParFXDYX,
    BParFXLimiter,
    BParFXSurroundPanner,
    BParFXShaper,
    BParFXDistortion,
    BParFXStereoSpread,
    BParFXLofi,
    BParFXSkreamer,
    BParFXRotator,
    BParFXTwang,
    BParFXCabinet,
    BParFX,
    BDyxMorphGroup,
    BDyxMorphMap,
    BProgram,
    BProgramContainer,
    BSample,
    VoiceGroup,
    BZone,
    BZoneLevelEnv,
    BZoneArraySer,
    BGroupCompleteSer,
    PresetImpl,
    VoiceGroups(VoiceGroups),
    GroupList(GroupList),
    ZoneList,
    PrivateRawObject,
    ProgramList,
    SlotList,
    StarCritList,
    LoopArray,
    BParameterArraySerBParFX8,
    BParameterArraySerBParInternalMod16,
    BParameterArraySerBParExternalMod32,
    BOutputConfiguration,
    FileNameListPreK51,
    FNTablePreK51,
    BParEnvAhdsr,
    BParEnvFm7,
    BParEnvDbd,
    BParFXTape,
    BParFXTrans,
    BParFXSSLGEQ,
    BInsertBus,
    BParFXSSLGBusComp,
    SaveSettings,
    BParGroupDynamics,
    FNTableImpl(FNTableImpl),
    FileNameList,
    BParFXFBComp,
    BParFXJump,
    QuickBrowseData,
    BSnapshot,
    BGroupSnapshot,
    BSnapshotMetaData,
    BParFXVan51,
    BParFXACBox,
    BParFXHotSolo,
    BParFXBassInvader,
    BParFXCat,
    BParFXDStortion,
    BParFXPlateReverb,
    BParFXCryWah,
    BParFXReplikaDelay,
    BParFXPhasis,
    BParFXFlair,
    BParFXChoral,
    BParFXCoreCell,
    BParFXHilbertLimiter,
    BParFXGaloisReverb,
    BParFXSupercharger,
    BParFXBassPro,
    BParFXPsycheDelay,
    BParFXRingModulator,
    Unsupported(u16),

    Program(Program),
    StructuredObject(StructuredObject),
}

impl TryFrom<&Chunk> for KontaktObject {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<KontaktObject, Error> {
        Ok(match chunk.id {
            // 0x54 BParFXBassInvader?,
            0x00 => KontaktObject::BParModBase,
            0x01 => KontaktObject::BAutomationObject,
            0x02 => KontaktObject::OutputPartition,
            0x03 => KontaktObject::BBank(chunk.try_into()?),
            0x04 => KontaktObject::BGroup,
            0x05 => KontaktObject::BLoop,
            0x06 => KontaktObject::BParScript,
            0x07 => KontaktObject::BParEnv,
            0x08 => KontaktObject::BParLFO,
            0x09 => KontaktObject::BParArp,
            0x0a => KontaktObject::BParEnvF,
            0x0b => KontaktObject::BParGlide,
            0x0c => KontaktObject::BParExternalMod,
            0x0d => KontaktObject::BParInternalMod,
            0x0e => KontaktObject::BParSrcMode,
            0x0f => KontaktObject::BParStartCriteria,
            0x10 => KontaktObject::BParFXDelay,
            0x11 => KontaktObject::BParFXChorus,
            0x12 => KontaktObject::BParFXFlanger,
            0x13 => KontaktObject::BParFXGainer,
            0x14 => KontaktObject::BParFXPhaser,
            0x15 => KontaktObject::BParFXReverb,
            0x16 => KontaktObject::BParFXIRC,
            0x17 => KontaktObject::BParFXSendLevel,
            0x18 => KontaktObject::BParFXFilter,
            0x19 => KontaktObject::BParFXCompressor,
            0x1a => KontaktObject::BParFXInverter,
            0x1b => KontaktObject::BParFXDYX,
            0x1c => KontaktObject::BParFXLimiter,
            0x1d => KontaktObject::BParFXSurroundPanner,
            0x1e => KontaktObject::BParFXDistortion,
            0x1f => KontaktObject::BParFXStereoSpread,
            0x20 => KontaktObject::BParFXLofi,
            0x21 => KontaktObject::BParFXSkreamer,
            0x22 => KontaktObject::BParFXRotator,
            0x23 => KontaktObject::BParFXTwang,
            0x24 => KontaktObject::BParFXCabinet,
            0x25 => KontaktObject::BParFX,
            0x26 => KontaktObject::BDyxMorphGroup,
            0x27 => KontaktObject::BDyxMorphMap,
            0x28 => KontaktObject::Program(chunk.try_into()?),
            0x29 => KontaktObject::BProgramContainer,
            0x2a => KontaktObject::BSample,
            0x2b => KontaktObject::VoiceGroup,
            0x2c => KontaktObject::BZone,
            0x2d => KontaktObject::BZoneLevelEnv,
            0x2e => KontaktObject::BZoneArraySer,
            0x2f => KontaktObject::BGroupCompleteSer,
            0x30 => KontaktObject::PresetImpl,
            0x32 => KontaktObject::VoiceGroups(chunk.try_into()?),
            0x33 => KontaktObject::GroupList(chunk.try_into()?),
            0x34 => KontaktObject::ZoneList,
            0x35 => KontaktObject::PrivateRawObject,
            0x36 => KontaktObject::ProgramList,
            0x37 => KontaktObject::SlotList,
            0x38 => KontaktObject::StarCritList,
            0x39 => KontaktObject::LoopArray,
            0x3a => KontaktObject::BParameterArraySerBParFX8,
            0x3b => KontaktObject::BParameterArraySerBParInternalMod16,
            0x3c => KontaktObject::BParameterArraySerBParExternalMod32,
            0x3d => KontaktObject::FileNameListPreK51,
            0x3e => KontaktObject::BOutputConfiguration,
            0x3f => KontaktObject::BParEnvAhdsr,
            0x40 => KontaktObject::BParEnvFm7,
            0x41 => KontaktObject::BParEnvDbd,
            0x42 => KontaktObject::BParFXTape,
            0x43 => KontaktObject::BParFXTrans,
            0x44 => KontaktObject::BParFXSSLGEQ,
            0x45 => KontaktObject::BInsertBus,
            0x46 => KontaktObject::BParFXSSLGBusComp,
            0x47 => KontaktObject::SaveSettings,
            0x4a => KontaktObject::BParGroupDynamics,
            0x4b => KontaktObject::FNTableImpl(chunk.try_into()?),
            0x4c => KontaktObject::BParFXFBComp,
            0x4d => KontaktObject::BParFXJump,
            0x4e => KontaktObject::QuickBrowseData,
            0x4f => KontaktObject::BSnapshot,
            0x50 => KontaktObject::BGroupSnapshot,
            0x51 => KontaktObject::BSnapshotMetaData,
            0x52 => KontaktObject::BParFXVan51,
            0x53 => KontaktObject::BParFXACBox,
            0x54 => KontaktObject::BParFXHotSolo,
            0x55 => KontaktObject::BParFXCat,
            0x56 => KontaktObject::BParFXDStortion,
            0x57 => KontaktObject::BParFXPlateReverb,
            0x58 => KontaktObject::BParFXCryWah,
            0x59 => KontaktObject::BParFXGaloisReverb,
            0x5a => KontaktObject::BParFXReplikaDelay,
            0x5b => KontaktObject::BParFXPhasis,
            0x5c => KontaktObject::BParFXFlair,
            0x5d => KontaktObject::BParFXChoral,
            0x5e => KontaktObject::BParFXCoreCell,
            0x5f => KontaktObject::BParFXHilbertLimiter,
            0x60 => KontaktObject::BParFXSupercharger,
            0x61 => KontaktObject::BParFXBassPro,
            0x63 => KontaktObject::BParFXPsycheDelay,
            0x64 => KontaktObject::BParFXRingModulator,
            _ => KontaktObject::Unsupported(chunk.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/StructuredObject/0x28")?;
        let data = Chunk::read(file)?;
        let chunk: KontaktObject = (&data).try_into()?;

        dbg!(chunk);

        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/KontaktV42/FNTableImpl/FNTableImpl-001")?;
        let data = Chunk::read(file)?;
        let chunk: KontaktObject = (&data).try_into()?;

        dbg!(chunk);

        Ok(())
    }
}
