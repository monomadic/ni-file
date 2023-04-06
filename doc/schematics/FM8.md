# FM8

## FM8Program

- The main preset file

| Offset | Length | Type         | Meaning       | Default    | Examples |
|--------|--------|--------------|---------------|------------|----------|
| 0      | 4      | uint32_t     | magic         | 0x464d3845 | 'FM8E'   |
| 4      | 4      | uint32_t     | majorVersion  | 0xCF (207) |          |
| 8      | 4      | uint         | minorVersion? | ?BC230000  | 9148     |
| 12     | -      | basic_string | name          |            |          |
| 12     | -      | basic_string |               |            |          |
| 12     | -      | basic_string |               |            |          |
| 12     | -      | basic_string |               |            |          |
