
void NI::SOUND::BinaryChunk::exportProperties(BinaryChunk *this, Properties *properties) {
    int *refCount;
    int currentRefCount;
    char *propName = "binary-chunk-size";
    ushort *propNameStr;

    GP::StringASCII::StringASCII((StringASCII *)&propNameStr, (CharSequence *)&propName);
    Properties::append<unsigned_long_long>(properties, propNameStr, (uint64_t *)(this + 8));

    if ((undefined4 *)(propNameStr - 6) !=
        &std::basic_string<unsigned_short, std::char_traits<unsigned_short>, std::allocator<unsigned_short>>::_Rep::_S_empty_rep_storage) {
        LOCK();
        refCount = (int *)(propNameStr - 2);
        currentRefCount = *refCount;
        *refCount = currentRefCount - 1;
        if (currentRefCount < 1) {
            delete (propNameStr - 6);
        }
    }
}
