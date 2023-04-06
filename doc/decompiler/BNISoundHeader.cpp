
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

ResultType BNISoundHeader::readItem(BNISoundHeader *this, Stream *stream, ReadContext *ctx) {
    bool isSuccess;
    int bytesRead;
    ResultType localResult;
    ItemFrameReader itemFrame;
    ResultType result;

    ItemFrameReader::ItemFrameReader(&itemFrame, stream, ctx);
    result.value = Item::readItem((Item *)this, stream, ctx);
    isSuccess = ResultType::operator.cast.to.bool(&result);

    if (isSuccess) {
        bytesRead = Stream::readRaw(stream, &(this->item).field_0x8, 0xde);
				// if correct amount of bytes were not read, return error
        if (bytesRead != 0xde) {
            ResultType::ResultType((ResultType *)&localResult, 8);
            result.value = localResult.value;
        }
    }

		// destroy temporary item frame
    ItemFrame::~ItemFrame((ItemFrame *)&itemFrame);

    return result;
}
