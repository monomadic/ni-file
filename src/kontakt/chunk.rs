use crate::{read_bytes::ReadBytesExt, Error};

use super::{
    objects::{
        BParFX, BParFXSendLevels, BParScript, BParamArrayBParFX8, Bank, FNTableImpl, FXDelay,
        FileNameListPreK51, GroupList, InsertBus, InternalMod, InternalModArray16, LoopArray,
        PrivateRawObject, Program, ProgramContainer, ProgramList, QuickBrowseData, SaveSettings,
        SlotList, StartCriteriaList, VoiceGroup, VoiceGroups, ZoneList,
    },
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

#[derive(Debug)]
pub enum KontaktObject {
    BParModBase,
    BAutomationObject,
    OutputPartition,
    Bank(Bank),
    BGroup,
    BLoop,
    BParScript(BParScript),
    BParEnv,
    BParLFO,
    BParArp,
    BParEnvF,
    BParGlide,
    BParExternalMod,
    BParInternalMod(InternalMod),
    BParSrcMode,
    BParStartCriteria,
    BParFXDelay(FXDelay),
    BParFXChorus,
    BParFXFlanger,
    BParFXGainer,
    BParFXPhaser,
    BParFXReverb,
    BParFXIRC,
    BParFXSendLevel(BParFXSendLevels),
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
    BParFX(BParFX),
    BDyxMorphGroup,
    BDyxMorphMap,
    BProgram,
    BProgramContainer(ProgramContainer),
    BSample,
    VoiceGroup(VoiceGroup),
    BZone,
    BZoneLevelEnv,
    BZoneArraySer,
    BGroupCompleteSer,
    PresetImpl,
    VoiceGroups(VoiceGroups),
    GroupList(GroupList),
    ZoneList(ZoneList),
    PrivateRawObject(PrivateRawObject),
    ProgramList(ProgramList),
    SlotList(SlotList),
    StartCriteriaList(StartCriteriaList),
    LoopArray(LoopArray),
    BParameterArraySerBParFX8(BParamArrayBParFX8),
    BParameterArraySerBParInternalMod16(InternalModArray16),
    BParameterArraySerBParExternalMod32,
    BOutputConfiguration,
    FileNameListPreK51(FileNameListPreK51),
    FNTablePreK51,
    BParEnvAhdsr,
    BParEnvFm7,
    BParEnvDbd,
    BParFXTape,
    BParFXTrans,
    BParFXSSLGEQ,
    BInsertBus(InsertBus),
    BParFXSSLGBusComp,
    SaveSettings(SaveSettings),
    BParGroupDynamics,
    FNTableImpl(FNTableImpl),
    FileNameList,
    BParFXFBComp,
    BParFXJump,
    QuickBrowseData(QuickBrowseData),
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
            0x03 => KontaktObject::Bank(chunk.try_into()?),
            0x04 => KontaktObject::BGroup,
            0x05 => KontaktObject::BLoop,
            0x06 => KontaktObject::BParScript(chunk.try_into()?),
            0x07 => KontaktObject::BParEnv,
            0x08 => KontaktObject::BParLFO,
            0x09 => KontaktObject::BParArp,
            0x0a => KontaktObject::BParEnvF,
            0x0b => KontaktObject::BParGlide,
            0x0c => KontaktObject::BParExternalMod,
            0x0d => KontaktObject::BParInternalMod(chunk.try_into()?),
            0x0e => KontaktObject::BParSrcMode,
            0x0f => KontaktObject::BParStartCriteria,
            0x10 => KontaktObject::BParFXDelay(chunk.try_into()?),
            0x11 => KontaktObject::BParFXChorus,
            0x12 => KontaktObject::BParFXFlanger,
            0x13 => KontaktObject::BParFXGainer,
            0x14 => KontaktObject::BParFXPhaser,
            0x15 => KontaktObject::BParFXReverb,
            0x16 => KontaktObject::BParFXIRC,
            0x17 => KontaktObject::BParFXSendLevel(chunk.try_into()?),
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
            0x25 => KontaktObject::BParFX(chunk.try_into()?),
            0x26 => KontaktObject::BDyxMorphGroup,
            0x27 => KontaktObject::BDyxMorphMap,
            0x28 => KontaktObject::Program(chunk.try_into()?),
            0x29 => KontaktObject::BProgramContainer(chunk.try_into()?),
            0x2a => KontaktObject::BSample,
            0x2b => KontaktObject::VoiceGroup(chunk.try_into()?),
            0x2c => KontaktObject::BZone,
            0x2d => KontaktObject::BZoneLevelEnv,
            0x2e => KontaktObject::BZoneArraySer,
            0x2f => KontaktObject::BGroupCompleteSer,
            0x30 => KontaktObject::PresetImpl,
            0x32 => KontaktObject::VoiceGroups(chunk.try_into()?),
            0x33 => KontaktObject::GroupList(chunk.try_into()?),
            0x34 => KontaktObject::ZoneList(chunk.try_into()?),
            0x35 => KontaktObject::PrivateRawObject(chunk.try_into()?),
            0x36 => KontaktObject::ProgramList(chunk.try_into()?),
            0x37 => KontaktObject::SlotList(chunk.try_into()?),
            0x38 => KontaktObject::StartCriteriaList(chunk.try_into()?),
            0x39 => KontaktObject::LoopArray(chunk.try_into()?),
            0x3a => KontaktObject::BParameterArraySerBParFX8(chunk.try_into()?),
            0x3b => KontaktObject::BParameterArraySerBParInternalMod16(chunk.try_into()?),
            0x3c => KontaktObject::BParameterArraySerBParExternalMod32,
            0x3d => KontaktObject::FileNameListPreK51(chunk.try_into()?),
            0x3e => KontaktObject::BOutputConfiguration,
            0x3f => KontaktObject::BParEnvAhdsr,
            0x40 => KontaktObject::BParEnvFm7,
            0x41 => KontaktObject::BParEnvDbd,
            0x42 => KontaktObject::BParFXTape,
            0x43 => KontaktObject::BParFXTrans,
            0x44 => KontaktObject::BParFXSSLGEQ,
            0x45 => KontaktObject::BInsertBus(chunk.try_into()?),
            0x46 => KontaktObject::BParFXSSLGBusComp,
            0x47 => KontaktObject::SaveSettings(chunk.try_into()?),
            0x4a => KontaktObject::BParGroupDynamics,
            0x4b => KontaktObject::FNTableImpl(chunk.try_into()?),
            0x4c => KontaktObject::BParFXFBComp,
            0x4d => KontaktObject::BParFXJump,
            0x4e => KontaktObject::QuickBrowseData(chunk.try_into()?),
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
        let file =
            File::open("tests/data/Objects/Kontakt/0x28-Program/ProgramV80/ProgramV80-000.kon")?;
        let data = Chunk::read(file)?;
        let _chunk: KontaktObject = (&data).try_into()?;

        Ok(())
    }

    #[test]
    fn test_fntableimpl() -> Result<(), Error> {
        let file = File::open("tests/data/Objects/Kontakt/0x4B-FNTableImpl/FNTableImpl-000.kon")?;
        let data = Chunk::read(file)?;
        let _chunk: KontaktObject = (&data).try_into()?;

        Ok(())
    }
}
