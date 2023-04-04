byte K4PatchLib::BCheckPatchHeader::GetHeaderVersionPreNIS(BFile *file)
{
    int iVar3;
    undefined4 currentPosition;
    undefined4 headerSignature;
    uint headerVersion;

    currentPosition = file->seek(0, 0);
    file->read(&headerSignature, 0x24);
    file->seek(currentPosition, 0);

    iVar3 = 0;
    do {
        if (headerSignature < 0x7fa89012) {
            if (headerSignature < 0x74b5a69b) {
                if (headerSignature < 0x10874353) {
                    if ((headerSignature == 0xa4d6e55a) || (headerSignature == 0xab85ef01) || (headerSignature == 0xb36ee55e)) {
                        if (headerVersion < 0x100) {
                            return 1;
                        }
                        return 0x10f < headerVersion | 2;
                    }
                } else if (headerSignature == 0x10874353) {
                    goto LAB_00068467;
                }
            } else if (headerSignature == 0x74b5a69b) {
                goto LAB_00068467;
            }
        } else if (headerSignature == 0x7fa89012) { // k2
            goto LAB_00068467;
        }

        headerSignature = (headerSignature << 8) | (headerSignature >> 24);
        headerVersion = (uint)(ushort)((headerVersion << 8) | (headerVersion >> 8));
        iVar3++;

        if (1 < iVar3) {
            return 0;
        }

    LAB_00068467: continue;
    } while (true);
}
