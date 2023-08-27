# StructuredObject

A generic container type that contains public data, private data, and children.

- LibKIO `StructuredObject`
- Kontakt 7
  - `SER::Read<>`
  - `SER::CHUNKED::Read<>`
  - BSerializable
  - BSerializablePreset
  - BSerializableStructured

| Offset | Length | Type     | Name              | Example | Notes                       |
| ------ | ------ | -------- | ----------------- | ------- | --------------------------- |
| 0x00   | 0x02   | uint16_t | id                | 0x25    |                             |
| 0x02   | 0x04   | uint32_t | length            |         | length of entire object     |
| 0x06   | 0x01   | bool     | isDataStructured  | true    | if false, read `length - 1` |
|        |        |          |                   |         | bytes and store in pubData  |
| 0x07   | 0x02   | uint16_t | privateDataId     |         |                             |
| 0x09   | 0x04   | uint32_t | privateDataLength |         |                             |
|        |        |          | privateData       |         |                             |
|        | 0x04   | uint32_t | pubDataLength     |         |                             |
|        |        |          | pubData           |         |                             |
|        | 0x04   | uint32_t | childDataLength   |         |                             |
|        |        |          | childData         |         |                             |

## PubData IDs

| ID   | Type           | Known Versions | Notes                |
| ---- | -------------- | -------------- | -------------------- |
| 0x25 | StructuredData |                | default version 0x52 |
| 0x28 | ProgramData    | 0x80           | -                    |
| 0x3D | ?              | 0x00           | -                    |
