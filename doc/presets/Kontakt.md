# Kontakt

Kontakt 1 and 2 files are XML documents, both with different schemas. Kontakt 4.2+ is a structured binary format similar to RIFF chunks.

## Kontakt Chunks

Chunks start with a 16-bit id, internally these are referred to as `SerType` or Serialization Type. Then followed by 32-bit length, then the data.

```
ID | SIZE | DATA
```

### SerTypes

- 0x00 BParModBase
- 0x01 BAutomationObject
- 0x02 OutputPartition
- 0x03 BBank
- 0x04 BGroup
- 0x05 BLoop
- 0x06 BParScript
- 0x07 BParEnv
- 0x08 BParLFO
- 0x09 BParArp
- 0x0a BParEnvF
- 0x0b BParGlide
- 0x0c BParExternalMod
- 0x0d BParInternalMod
- 0x0e BParSrcMode
- 0x0f BParStartCriteria
- 0x10 BParFXDelay
- 0x11 BParFXChorus
- 0x12 BParFXFlanger
- 0x13 BParFXGainer
- 0x14 BParFXPhaser
- 0x15 BParFXReverb
- 0x16 BParFXIRC
- 0x17 BParFXSendLevel
- 0x17 BParFXSendLevels
- 0x18 BParFXFilter
- 0x19 BParFXCompressor
- 0x1a BParFXInverter
- 0x1b BParFXDYX
- 0x1c BParFXLimiter
- 0x1d BParFXSurroundPanner
- 0x1d BParFXShaper
- 0x1e BParFXDistortion
- 0x1f BParFXStereoSpread
- 0x20 BParFXLofi
- 0x21 BParFXSkreamer
- 0x22 BParFXRotator
- 0x23 BParFXTwang
- 0x24 BParFXCabinet
- 0x25 BParFX
- 0x26 BDyxMorphGroup
- 0x27 BDyxMorphMap
- 0x28 BProgram
- 0x29 BProgramContainer
- 0x2a BSample
- 0x2b ? VoiceGroup
- 0x2c BZone
- 0x2d BZoneLevelEnv
- 0x2e BZoneArraySer
- 0x2f BGroupCompleteSer
- 0x30 PresetImpl
- 0x32 ? VoiceGroups
- 0x33 ? GroupList
- 0x34 ? ZoneList
- 0x35 ? PrivateRawObject
- 0x36 ? ProgramList
- 0x37 ? SlotList
- 0x38 ? StarCritList
- 0x39 ? LoopArray
- 0x3a BParameterArraySer<BParFX,8>
- 0x3b BParameterArraySer<BParInternalMod,16>
- 0x3c BParameterArraySer<BParExternalMod,32>
- 0x3e BOutputConfiguration
- 0x3d FileNameListPreK1 / FNTablePreK51
- 0x3f BParEnv_AHDSR
- 0x40 BParEnv_FM7
- 0x41 BParEnv_DBD
- 0x42 BParFXTape
- 0x43 BParFXTrans
- 0x44 BParFXSSLGEQ
- 0x45 BInsertBus
- 0x46 BParFXSSLGBusComp
- 0x47 SaveSettings
- 0x48 ? PrivateRawObject
- 0x49 ? PrivateRawObject
- 0x4a BParGroupDynamics
- 0x4b FNTableImpl | FileNameList
- 0x4c BParFXFBComp
- 0x4d BParFXJump
- 0x4e QuickBrowseData
- 0x4f BSnapshot
- 0x50 BGroupSnapshot
- 0x51 BSnapshotMetaData
- 0x52 BParFXVan51
- 0x53 BParFXACBox
- 0x54 BParFXHotSolo
- 0x54 BParFXBassInvader
- 0x55 BParFXCat
- 0x56 BParFXDStortion
- 0x57 BParFXPlateReverb
- 0x58 BParFXCryWah
- 0x5a BParFXReplikaDelay
- 0x5b BParFXPhasis
- 0x5c BParFXFlair
- 0x5d BParFXChoral
- 0x5e BParFXCoreCell
- 0x5f BParFXHilbertLimiter
- 0x59 BParFXGaloisReverb
- 0x60 BParFXSupercharger
- 0x61 BParFXBassPro
- 0x63 BParFXPsycheDelay
- 0x64 BParFXRingModulator

Many of these are a `StructuredObject`, but not all. It is important to check the ID first.
