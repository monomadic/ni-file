# ProgramData

The main object of a patch. Contains some high-level data such as global volume, pan, tune, program name and various other paramaters relating to the use of the patch as a whole.

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

- size: 144 bytes

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

## VA5

### Public Parameters
