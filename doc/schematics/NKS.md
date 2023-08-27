# NKS Container

## Kontakt Objects

Ids are known as `SerType` or Serialization Type.

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
