bool NI::GP::FileContainer::isContainerFile(FileName *param_1, std::vector<tMetaHeader> *param_2) {
    FileStorage fileStorage;
    FileName fileName(param_1);
    bool fileOpened = fileStorage.open(fileName, 1, 0);

    if (!fileOpened) {
        return false;
    }

    uint32_t header[4];
    int bytesRead = fileStorage.read(header, sizeof(header));

    if (param_2 == nullptr) {
        if (bytesRead == sizeof(header) && isSignatureMatch(header)) {
            return true;
        }
        fileStorage.seek64(-bytesRead, SEEK_CUR);
    } else {
        while (bytesRead == sizeof(header)) {
            if (!isSignatureMatch(header)) {
                break;
            }

            param_2->push_back(*reinterpret_cast<tMetaHeader *>(header));

            if (someCondition(local_158, fileStorage, local_8c)) {
                return false;
            }

            bytesRead = fileStorage.read(header, sizeof(header));
        }

        if (bytesRead != sizeof(header)) {
            return false;
        }

        if (local_46c == 1) {
            bytesRead = fileStorage.read(local_3cc, 0x1e0);
            if (bytesRead != 0x1e0) {
                return false;
            }
        }

        return isSignatureMatch2(local_468, local_464, local_460, local_44c, local_448, local_444, local_440);
    }

    return false;
}

// ^ can be !=
bool isSignatureMatch(uint32_t header[4]) {
    return (header[0] ^ 0x4E205C2F | header[1] ^ 0x43462049) == 0 &&
           (header[2] ^ 0x44544D20 | header[3] ^ 0x5C2F2020) == 0;
}

bool someCondition(int local_158, FileStorage &fileStorage, int local_8c) {
    // Some condition not clear from the original decompiled code
}

bool isSignatureMatch2(int local_468, uint local_464, uint local_460, uint local_44c, uint local_448, uint local_444, uint local_440) {
    return local_468 == 0 &&
           (local_464 ^ 0xF0F0F0F0 | local_460 ^ 0xF0F0F0F0) == 0 &&
           (local_44c ^ 0x4E205C2F | local_448 ^ 0x43462049) == 0 &&
           (local_444 ^ 0x434F5420 | local_440 ^ 0x5C2F2020) == 0;
}
