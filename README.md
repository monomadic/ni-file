# Native Instruments File Format

## Block Format

File is made up of nested blocks, denoted with 'hsin' tags. Basic block format is:

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

## Data segments

Data blocks seem to also be nested.

```
[length]
["DSIN"] // data segment in
[type? le-u32]
[? le-u32 almost always 1]
[... new data segment]