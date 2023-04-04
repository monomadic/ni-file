void K4PO::ChunkData::doRead(ChunkData *this, Stream *fp)
{
    uint16_t version;
    int dataLength;

    version = fp->readU16();
    this->version = version;
    dataLength = fp->readS32();
    this->dataLength = dataLength;

    if (dataLength < 0) {
        throw BSerError("unexpected data chunk (file corrupt)", 3);
    } else {
        if ((fp->someVariable != 0) && (dataLength = *(int *)(*(int *)(fp->someVariable) + 8), dataLength != 0) && (dataLength != 3)) {
            return;
        }
        throw BSerError("could not read", 5);
    }
}
