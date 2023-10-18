# Program

A single instance of an instrument, containing settings, zones, effects, etc. Multi (NKM) presets contain multiple `Programs`.

| Offset | Length | Type     | Name              | Notes         |
| ------ | ------ | -------- | ----------------- | ------------- |
| 0x00   | 0x02   | uint16_t | serType           | always `0x28` |
| 0x02   | 0x04   | uint32_t | length            |               |
| 0x06   | 0x01   | bool     | isDataStructured  | always `true` |
| 0x07   | 0x02   | uint16_t | structVersion     |               |
| 0x09   | 0x04   | uint32_t | privateDataLength |               |
|        |        |          | privateData       |               |
|        | 0x04   | uint32_t | pubDataLength     |               |
|        |        |          | pubData           |               |
|        | 0x04   | uint32_t | childDataLength   |               |
|        |        |          | childData         |               |

## V80

### Public Parameters

| Offset | Length | Type    | Name                     | Notes |
| ------ | ------ | ------- | ------------------------ | ----- |
|        |        | wstring | name                     |       |
|        |        | float64 | num_bytes_samples_total  |       |
|        |        | int8    | transpose                |       |
|        |        | float32 | volume                   |       |
|        |        | float32 | pan                      |       |
|        |        | float32 | tune                     |       |
|        |        | uint8   | low_velocity             |       |
|        |        | uint8   | high_velocity            |       |
|        |        | uint8   | low_key                  |       |
|        |        | uint8   | high_key                 |       |
|        |        | int16   | default_key_switch       |       |
|        |        | int32   | dfd_channel_preload_size |       |
|        |        | int32   | library_id               |       |
|        |        | uint32  | fingerprint              |       |
|        |        | uint32  | loading_flags            |       |
|        |        | bool    | group_solo               |       |
|        |        | int32   | cat_icon_idx             |       |
|        |        | wstring | instrument_credits       |       |
|        |        | wstring | instrument_author        |       |
|        |        | wstring | instrument_url           |       |
|        |        | int16   | instrument_cat1          |       |
|        |        | int16   | instrument_cat2          |       |
|        |        | int16   | instrument_cat3          |       |

### Children

- `BParamArrayBParFX8`
- `BParamArrayBParFX8`
- `VoiceGroups`
- `GroupList`
  - .. `Group`

## VA5

### Public Parameters
