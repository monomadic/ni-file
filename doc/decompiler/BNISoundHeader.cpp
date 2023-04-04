
uint32_t BNISoundHeader::readItem(BNISoundHeader *this, Stream *stream, ReadContext *ctx) {
    bool success;
    int bytesRead;
    uint32_t localResult[2];
    ItemFrameReader frameReader[24];
    uint32_t result;

    NI::SOUND::ItemFrameReader::ItemFrameReader(frameReader, stream, ctx);
    result = NI::SOUND::Item::readItem((Item *)this, stream, ctx);
    success = NI::SOUND::ResultType::operator.cast.to.bool((ResultType *)&result);

    if (success) {
        bytesRead = NI::GP::Stream::readRaw(stream, this + 0x14, 0xde);
        if (bytesRead != 0xde) {
            NI::SOUND::ResultType::ResultType((ResultType *)localResult, 8);
            result = localResult[0];
        }
    }

    NI::SOUND::ItemFrame::~ItemFrame((ItemFrame *)frameReader);
    return result;
}
