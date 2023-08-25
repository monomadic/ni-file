# NKS Container

## Kontakt Objects

Ids are known as `SerType` or Serialization Type.

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
- 0x20 BParFXLoFi
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
- 0x3d FileNameListPreK51
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

- `GetHeaderVersionPreNIS(file);` returns:
  - 1: 36 bytes PreV2
  - 2: 170 bytes BPatchHeaderV2
  - 3: 222 bytes BPatchHeaderV42

## PreV2

- Size: 36 bytes

| Offset | Length | Type     | Meaning      | Example    | Notes |
| ------ | ------ | -------- | ------------ | ---------- | ----- |
| 0      | 4      | uint32_t | magic        | 0x5EE56EB3 |       |
| 0x04   | 0x04   | uint32_t | headerLength | 36         |       |
| 0x08   | 0x02   | uint16_t |              | 80         |       |
| 0x10   | 0x02   | uint16_t |              | 2          |       |
| 0x0C   | 0x04   | uint32_t |              | 0          |       |
| 0x10   | 0x04   | uint32_t |              | 0          |       |
| 0x14   | 0x04   | uint32_t |              | 1          |       |
| 0x18   | 0x04   | uint32_t |              | 0x68D2073E |       |
| 0x1C   | 0x04   | uint32_t |              |            |       |
| 0x20   | 0x04   | uint32_t |              |            |       |

## BPatchHeaderV2

- Size: 170 bytes
- Detection: `if (magic == 0xa4d6e55a || magic == 0xab85ef01 || magic == 0xb36ee55e || magic == 0x10874353 || magic == 0x74b5a69b || magic == 0x7fa89012) && fileVersion < 256`

| Offset | Length | Type     | Meaning        | Example    | Notes                                    |
| ------ | ------ | -------- | -------------- | ---------- | ---------------------------------------- |
| 0      | 4      | uint32_t | magic          | 0x1290A87F |                                          |
| 0x04   | 0x04   | uint32_t | length?        |            |                                          |
| 0x08   | 0x02   | uint16_t | headerVersion  | 0x0001     |                                          |
| 0x0A   | 0x04   | uint32_t | headerMagic    | 0x722A013E |                                          |
| 0x0E   | 0x02   | uint16_t | patchtype      | 0x1 (nki)  | 0=nkm, 1=nki, 2=nkb, 3=nkp, 4=nkg, nkz=5 |
| 0x10   | 0x04   | AppVersi | appVersion     | 0x02010002 | 0x02010002=2.1.2                         |
| 0x14   | 0x04   | uint32_t | appSignature   | 0x4B34504C | "Kon2"                                   |
| 0x18   | 0x04   | time_t   | createdAt      |            |                                          |
| 0x1C   | 0x04   |          | ?              | 0x96020000 | 662, 1122                                |
| 0x20   | 0x02   | uint16_t | numZones       |            |                                          |
| 0x22   | 0x02   | uint16_t | numGroups      |            |                                          |
| 0x24   | 0x02   | uint16_t | numInstruments |            |                                          |
| 0x26   | 0x02   | uint16_t |                |            | 1                                        |
| 0x28   | 0x04   | uint32_t | length?        |            |                                          |

## BPatchHeaderV42

- Size: 222 bytes
- - Detection: `if (magic == 0xa4d6e55a || magic == 0xab85ef01 || magic == 0xb36ee55e || magic == 0x10874353 || magic == 0x74b5a69b || magic == 0x7fa89012) && fileVersion >= 271`

| Offset | Length | Type     | Meaning            | Example    | Notes                                                      |
| ------ | ------ | -------- | ------------------ | ---------- | ---------------------------------------------------------- |
| 0x00   | 0x04   | uint32_t | magic              | 0x1290A87F | 0xa4d6e55a, 0xab85ef01, 0xb36ee55e, 0x10874353, 0x74b5a69b |
| 0x04   | 0x04   | uint32_t | zLibLength         |            | Internal preset compressed size                            |
| 0x08   | 0x02   | uint16_t | headerVersion      | 0x1001     | Found 272                                                  |
| 0x0A   | 0x04   | uint32_t | headerMagic        | 0x1A6337EA |                                                            |
| 0x0E   | 0x02   | uint16_t | patchtype          | 0x1 (nki)  | 0=nkm, 1=nki, 2=nkb, 3=nkp, 4=nkg, nkz=5                   |
| 0x10   | 0x04   | AppVersi | appVersion         | 0x50500FF  | 0x5050FF=5.5.256                                           |
| 0x14   | 0x04   | uint32_t | appSignature       | 0x4B34504C | "Kon4"                                                     |
| 0x18   | 0x04   | time_t   | createdAt          |            |                                                            |
| 0x1C   | 0x04   |          |                    |            | 0                                                          |
| 0x20   | 0x02   | uint16_t | numZones           |            |                                                            |
| 0x22   | 0x02   | uint16_t | numGroups          |            |                                                            |
| 0x24   | 0x02   | uint16_t | numInstruments     |            |                                                            |
| 0x26   | 0x10   |          |                    |            |                                                            |
| 0x36   | 0x10   | uint32_t | icon               |            | 0x1C is "New"                                              |
|        |        |          |                    |            |                                                            |
| 0xA2   | 0x10   |          | checksum           |            | OpenSSL(?) EVP MD5                                         |
| 0xB2   | 0x04   | uint32_t | appSVNRev          |            |                                                            |
| 0xB6   | 0x04   | uint32_t |                    |            |                                                            |
| 0xBA   | 0x04   | uint32_t | decompressedLength |            |                                                            |
|        | 0x20   |          |                    |            |                                                            |
