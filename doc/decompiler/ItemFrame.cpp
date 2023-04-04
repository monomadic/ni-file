ssize_t __thiscall NI::SOUND::ItemFrame::read(ItemFrame *this,Stream *stream,void *unused,size_t unused2) {
  this->size = GP::Stream::readU64(stream);
  this->domainID = GP::Stream::readU32(stream);
  this->itemID = GP::Stream::readU32(stream);
  this->version = GP::Stream::readU32(stream);
  return 1;
}

bool NI::SOUND::ItemFrame::isFrameForBase(ItemFrame *this)
{
    if (this->domainID == "NISD" && this->itemID == 1) {
        return true;
    }

    return false;
}
