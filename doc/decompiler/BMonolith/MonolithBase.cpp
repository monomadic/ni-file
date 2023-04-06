bool K4PatchLib::BMonolith::MonolithBase::LocateItem(BFile *file, BFileName *fileName, int *index) {
    if (!fileName->IsEmpty()) {
        return this->field0_0x0->FindItem(this, file, fileName, index);
    }
    return false;
}

int* K4PatchLib::BMonolith::MonolithBase::GetPatch() {
    return &patch;
}

uint32_t K4PatchLib::BMonolith::MonolithBase::LocatePatch(MonolithBase *this, BFile *file, BFileName *fileName, int *param_3) {
    if (fileName->IsEmpty()) {
        return 0;
    }

    if (!this->FolderNumItems(file, fileName, param_3)) {
        return 0;
    }

    file->SetPosition(*param_3, 0);

    if (file->Read(&this->field32_0x23, 0x1b) != 0x1b) {
        return 0;
    }

    if (this->field32_0x23 != 0x4916e63c) {
        swapEndian(&this->field32_0x23);
        swapEndian(&this->field33_0x27);
        swapBytes(&this->field_0x29, &this->field_0x2c);
        swapBytes(&this->field_0x2a, &this->field_0x2b);
        swapBytes(&this->field_0x2d, &this->field_0x30);
        swapBytes(&this->field_0x2e, &this->field_0x2f);
        swapBytes(&this->field_0x32, &this->field_0x35);
        swapBytes(&this->field_0x33, &this->field_0x34);
        swapBytes(&this->field_0x36, &this->field_0x39);
        swapBytes(&this->field_0x37, &this->field_0x38);
        swapBytes(&this->field_0x3a, &this->field_0x3d);
        swapBytes(&this->field_0x3b, &this->field_0x3c);

        if (this->field32_0x23 != 0x4916e63c) {
            return 0;
        }
    }

    if (this->field33_0x27 >= 0x111) {
        return 0;
    }

    *param_3 += 0x1b;
    return 1;
}

int K4PatchLib::BMonolith::MonolithBase::FolderNumItems(MonolithBase *this, BFile *file, BFileName *fileName, ushort param_3, int param_4) {
    int count = 0;

    if (param_3 == 0) {
        return 0;
    }

    if (!readHeader(this, file, fileName, &param_4)) {
        return 0;
    }

    moveToOffset(file, param_4, 0);

    if (!readData(file, &uStack_2c, 0x16)) {
        return 0;
    }

    if (!validateHeader(uStack_2c)) {
        return 0;
    }

    if (uStack_28 > 0x110) {
        return 0;
    }

    uint dataLength = uStack_1e * 10;
    auto dataBuffer = new uint8_t[dataLength];

    if (readData(file, dataBuffer, dataLength) == dataLength) {
        if (uStack_2c != 0x5e70ac54) {
            swapDataBytes(dataBuffer, uStack_1e);
        }

        count = countMatchingItems(dataBuffer, uStack_1e, param_3);
    }

    delete[] dataBuffer;

    return count;
}
