# Native Instruments File Format

## Block Format

File is made up of nested blocks, denoted with 'hsin' tags. Basic block format is:

### HSIN - Header Section In

```
[block-length: le-u64]
[? block-type?: le-u32]
["hsin"] // header segment in
[? unknown: le-u64]
[? checksum (16-bytes)] // unsure what type of checksum, changes with every save (date included somewhere in nested blocks?)
[content-block (block-length bytes)]
[? unknown  le-u32, almost always 1]
[child-blocks: le-u32]
[? unknown le-32 (usually 0)]
[8 byte tag]
```

For example, `[2557][1]["hsin"][1]["ÅÈ¿K 0ÉGlUG"][...][1][1]["4KIN", 3]`

The first block length in the file will usually be the entire file size, as it represents one block. Child blocks are nested inside this.

I think this schematic is slightly wrong, as the main block (the one that wraps the rest of the file) seems to only have 2 u32s at the end and not a final tag. It is possible these additional tags are not accounted for in file sizes and instead just parsed as tokens.

Most files appear to have a main segment (the same across all file types), then a single child that describes the type of file it is (kontakt, massive, etc), and has the version number string in it, and then this segment has 4 or 5 children, one of which is the preset (compressed), one of which is the 'library database metadata', another short one which is almost empty, and then the author data (this seems to be decompressed metadata from the preset itself, and if the data doesn't match the compressed versions it will throw an error).

### DSIN - Data Structure In

DSIN blocks act as maps or slice indexes. The first DSIN is usually the length of the whole data chunk, minus footer data.

format guess new
[offset]
[structure-type: [DSIN/4KIN][u32][1]]

Format:
```
[offset to data start until end of buffer]
["DSIN"] // data segment in
[type? le-u32]
[? le-u32 almost always 1]
[... new data segment]
```

Footer data

## Compressed segments

The main preset is compressed with a custom LZ77 variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a data segment.

IMPORTANT: the compression starts 16 bytes just after the last DSIN of the segment (eg. "DSIN"+16 bytes). This is usually a 01h. There is often 20 bytes between the "hsin" and the start of the compression, but given it is compressed data this should not be relied upon.

## Data

Strings seem to be sometimes LE-UTF16. Sometimes they seem to be terminated with nulls but often not - sometimes they are length delimited (length, then the string values).

DSINs (44 53 49 4E 73 00 00 00) - compressed segment
DSINe (44 53 49 4E 65 00 00 00) - string? (eg:

    file version string 01 00 00 00 00 02 00 00 00 01 00 00 00 08 00 00
                        00 35 00 2E 00 36 00 2E 00 30 00 2E 00 34 00 36 00

DSIN1 (44 53 49 4E 01 00 00 00) - last element (?)
DSINj () - 01 00 00 00 02 00 00 00

    kontakt header
    01 00 00 00 02 00 00 00
    
    header string
    01 00 00 00 01 00 00 00 01 00 00 00 00 00
    00 00 00 00 00 00 00 00 00 00 0D 62 65 85

## Questions

- how does DSIN work? They appear to use offsets within the section?
- how is a compressed section marked as such?
- how are checksums calculated? They seem to change with each save, is there a date format embedded somewhere?

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to DSIN tags. I think NI might have tried to make their code more efficient by directly reading offsets.
