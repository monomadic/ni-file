byte K4PatchLib::BCheckPatchHeader::GetHeaderVersionPreNIS(BFile *file) {
    uint32_t filePosition;
    int iteration;
    uint32_t magic;
    ushort local_2c;

    // Save the current file position
    filePosition = file->tell();

    // Move to the start of the file
    file->seek(0, 0);

    // Read magic number (4 bytes) from the file
    file->read(&magic, 0x24);

    // Move back to the original file position
    file->seek(filePosition, 0);

    iteration = 0;
    do {
        // Check if the magic number matches any of the known versions
        if (magic == 0xa4d6e55a || magic == 0xab85ef01 || magic == 0xb36ee55e ||
            magic == 0x10874353 || magic == 0x74b5a69b || magic == 0x7fa89012) {
            if (local_2c < 0x100) {
                return 1;
            }
            return 0x10f < local_2c | 2;
        }

        // Rotate magic number (to handle endianness) and update local_2c
        magic = (magic << 24) | (magic >> 8);
        local_2c = (local_2c >> 8) | (local_2c << 8);

        // Increment iteration count
        iteration++;

				// only iterate once (little/big endian)
        if (iteration > 1) {
            return 0;
        }
    } while (true);
}


uint32_t K4PatchLib::BCheckPatchHeader::GetHeaderSizePreNIS(BFile *file) {
    int headerVersion;
    uint32_t headerSize;

    // Get the header version
    headerVersion = GetHeaderVersionPreNIS(file);

    // Determine header size based on the header version
    switch (headerVersion) {
        case 1:
            headerSize = 36;
            break;
        case 2:
            headerSize = 170;
            break;
        case 3:
            headerSize = 222;
            break;
        default:
            headerSize = 0;
            break;
    }

    return headerSize;
}

uint32_t K4PatchLib::BCheckPatchHeader::CheckMonolithPreNIS(BFileName *param_1) {
    BFile file;
    BFileName fileName;
    int i;
    uint32_t result;
    uint8_t buffer[170];

    // Initialize the file and file name objects
    BFile::BFile(&file);
    BFileName::BFileName(&fileName);

    // Open the file
    if (BFile::Open(&file, param_1, false, false, 0x58585858, NULL, NULL) == 0) {
        // Read 170 bytes from the file
        if (BFile::Read(&file, buffer, 170) == 170) {
            // Perform checks on the read data
            for (i = 0; i < 2; i++) {
                if (buffer[0] == 1) {
                    result = 1;
                    break;
                }

                // Shift the read data one byte to the left
                for (int j = 0; j < 169; j++) {
                    buffer[j] = buffer[j + 1];
                }
            }
        }
    }

    // Close the file and destroy the file name object
    BFile::~BFile(&file);
    BFileName::~BFileName(&fileName);

    return result;
}
