16-bit strings: 00 comes after. null-terminated by 00 00. spaces 00 20. (le)

HEADER: 210 bytes
- i32 file-size

generic to all NI files.

SUBHEADER: offset 210 (187 bytes)
- i32 remaining-file-size

specific to instrument/app

330: 01-kontakt

SECTION: offset 450
- i32 section-size (not including this i32, or 8 more bytes)

# PATTERN:
[section-size:i32][0:i32][0:i32][hsin]

# hsin - header section?
- always first block of file
- followed by 32 bytes
- char 'hsin'
- i32 1
- i32 0
- i32 0
- 16b binary data
- i32 150
- i32 0

# DSIN - data section?
- 14 bytes total
- char DSIN
- i32 118
- i32 1
- i32 72
- i32 0

# 4KIN
- 20 bytes
- char '4kin'
- i32 ? 3, 4
- i32 EOF LENGTH
- seem to give length markers until the end of the file

## VALUES

01 00 00 00 00 00 00 00 30 E1 F8 37 00 14 C0 46 A8 A2 00 0E C8 D5 20 91 6D 00 00 00 00 00 00 00

08 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 00 00 00 04 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00


# DSIN

DSIN <offset> <? 1> <size> <? 0>
