# Block Format

File is made up of nested blocks. Basic block format is:

```
[block-length: le-u64]
[? block-type?: le-u32]
["hsin"]
[? unknown: le-u64]
[? checksum (16-bytes)] // unsure what type of checksum, changes with every save (date included somewhere in nested blocks?)
[content-block (block-length bytes)]
[? unknown  le-u32, almost always 1]
[child-blocks: le-u32]
[? unknown le-32 (usually 0)]
[8 byte tag]
```

So the first block length will usually be the entire file size, as it represents one block.

