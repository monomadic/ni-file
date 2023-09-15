## NCW File Format Specification

The NCW file format consists of 3 main sections:

1. Main header (120 bytes)
2. Block address table
3. Audio data blocks

### 1. Main Header (120 bytes)

The main header contains metadata about the audio file. It has a fixed size of 120 bytes.

| Field                | Size     | Offset | Description                                         |
| -------------------- | -------- | ------ | --------------------------------------------------- |
| File signature       | 8 bytes  | 0      | 0x01A89ED631010000 or 0x01A89ED630010000            |
| Number of channels   | 2 bytes  | 8      | Number of audio channels (1 for mono, 2 for stereo) |
| Bits per sample      | 2 bytes  | 10     | Bit depth per sample (8, 16, 24, 32)                |
| Sample rate          | 4 bytes  | 12     | Sample rate in Hz                                   |
| Number of samples    | 4 bytes  | 16     | Total number of samples in file                     |
| Block address offset | 4 bytes  | 20     | File offset of block address table (part 2)         |
| Block data offset    | 4 bytes  | 24     | File offset of audio data blocks (part 3)           |
| Block data size      | 4 bytes  | 28     | Total size in bytes of audio data blocks            |
| Padding              | 88 bytes | 32     | Reserved padding, should be zero                    |

### 2. Block Address Table

The block address table contains the file offset of each audio data block, stored as 4-byte unsigned integers.

The size of the table is:

    Block data offset - Block address offset

And the number of entries is:

    (Block data offset - Block address offset) / 4

Each entry is the file offset of the start of an audio data block relative to the start of the audio data section (part 3).

### 3. Audio Data Blocks

The audio data is divided into blocks of up to 512 samples per channel. Each block has the following structure:

| Field           | Size    | Description                               |
| --------------- | ------- | ----------------------------------------- |
| Block signature | 4 bytes | 0x160C9A3E                                |
| Base value      | 4 bytes | Initial sample value for delta encoding   |
| Bits            | 2 bytes | Number of bits per sample used for block: |
|                 |         | - 0 = original bit depth                  |
|                 |         | - Negative = compressed bit depth         |
|                 |         | - Positive = delta encoding               |
| Flags           | 2 bytes | 1 = MID/SIDE encoding, 0 = L/R encoding   |
| Padding         | 4 bytes | Reserved, should be zero                  |

Followed by the audio sample data for the block.

**Compression:**

The audio data may be compressed via bit truncation. This is indicated by a negative value in the Bits field. The absolute value of Bits indicates the truncated bit depth used for the block.

**Delta encoding:**

If Bits is positive, the block contains delta values rather than direct samples. The Base value field contains the initial sample value. Each delta value is added to reconstruct the original samples.

**Channel encoding:**

If Flags = 1, the block contains Mid and Side channel data instead of Left and Right.

Left channel = Mid + Side
Right channel = Mid - Side

This provides a summary of the key structures and encoding used in the NCW file format. Let me know if any part needs additional clarification or expansion.
