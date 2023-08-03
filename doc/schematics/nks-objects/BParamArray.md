# BParamArray

An array container. Fixed sizes of 8, 16, and 32 are possible, (determined beforehand in the type signature).

- LibKIO `BParamArray<>`
  - `BParamArray<8>`
  - `BParamArray<16>`
  - `BParamArray<32>`
- Kontakt 7 `BParameterArraySer`
  - `BParameterArray<BParFX,8>`
  - `BParameterArray<BScriptParser,8>`
  - `BParameterArraySer<BParFX,8>`
  - `BParameterArraySer<BParInternalMod,16>`
  - `BParameterArraySer<BParExternalMod,32>`

| Offset | Length | Type     | Name    | Example    | Notes     |
| ------ | ------ | -------- | ------- | ---------- | --------- |
| 0x00   | 2      | uint16_t | id      | 0x3A       |           |
| 0x02   | 1      | bool     | ?       | false      | skipRead? |
| 0x03   | 2      | uint16_t | version | 0x10, 0x11 |           |

## Array Items

| Offset | Length | Type             | Name   | Example | Notes                     |
| ------ | ------ | ---------------- | ------ | ------- | ------------------------- |
| 0x02   | 1      | bool             | doRead | true    | Skip reading if `false`   |
|        | ?      | StructuredObject | object |         | StructuredObject::factory |
