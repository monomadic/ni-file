
uint32_t CheckForEncryption(void *patch) {
    uint32_t headerValue = 0;
    int readResult;
    uint32_t result;

    readResult = K4PL_ReadPatchHeaderValue(patch, 0x12, &headerValue);

    if (readResult == 0) {
        if ((headerValue & 4) == 0) {
            result = 2;
        } else {
            K4PL_ClosePatch(patch);
            _InitKIO(0);
            result = 1;
        }
    } else {
        K4PL_ClosePatch(patch);
        _InitKIO(0);
        result = 0x101;
    }

    return result;
}

uint32_t K4PL_ReadPatchHeaderValue(void *patch, int propertyId, uint *propertyValue) {
    int internalData;
    uint32_t result = 1;
    uint32_t tempValue;

    internalData = K4PL_DETAIL::InternalPatchData::GetData(patch);

    if ((internalData != 0) && (*(int *)(internalData + 0xd3) == -0x15c89ce6)) {
        result = 3;

        switch (propertyId) {
        case 1:
            tempValue = *(uint *)(internalData + 0xd9);
            break;
        case 2:
            tempValue = *(uint *)(internalData + 0xdd);
            break;
        case 3:
            tempValue = (uint)*(ushort *)(internalData + 0xe7);
            break;
        // ... Add other cases here ...
        default:
            return result;
        }

        *propertyValue = tempValue;
        result = 0;
    }

    return result;
}

int K4PL_DecodePatch(void *patch, unsigned char *data) {
    InternalPatchData *patchData;
    int result;
    int *structuredObj;
    ChunkData chunk1;
    ChunkData chunk2;

    patchData = (InternalPatchData *)K4PL_DETAIL::InternalPatchData::GetData(patch);
    if (patchData != nullptr && K4PL_DETAIL::InternalPatchData::ExtractPatchData(patchData, data) == 0) {
        Stream *stream = &patchData->stream;
        structuredObj = patchData->field8_0x8;
        if (structuredObj != nullptr) {
            (*structuredObj->Close)(structuredObj, 0, 0);
        }
        K4PO::ChunkData::doRead(&chunk1, stream);
        structuredObj = K4PO::StructuredObject::factory(chunk1.version, chunk1.dataLength);
        patchData->field395_0x1a8 = structuredObj;
        (*structuredObj->Read)(structuredObj, stream);

        structuredObj = patchData->field8_0x8;
        while (structuredObj != nullptr) {
            int objType = structuredObj[2];
            if (objType == 0 || objType == 3 || (structuredObj != nullptr && objType == 2)) {
                break;
            }
            K4PO::ChunkData::doRead(&chunk2, stream);
            structuredObj = K4PO::StructuredObject::factory(chunk2.version, chunk2.dataLength);

            if (*(short *)(structuredObj + 1) == 0x4b || *(short *)(structuredObj + 1) == 0x3d) {
                patchData->field396_0x1ac = structuredObj;
                (*structuredObj->Read)(structuredObj, stream);
                break;
            }
            (*structuredObj->Read)(structuredObj, stream);

            if (structuredObj != nullptr) {
                (*structuredObj->Free)(structuredObj);
            }
            structuredObj = patchData->field8_0x8;
        }
        result = patchData->field396_0x1ac != nullptr ? 0 : 0xc;
    } else {
        result = 1;
    }
    return result;
}
