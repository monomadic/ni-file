
undefined __thiscall
NI::SOUND::Properties::getValue(Properties *this,char *propertyName,String *value)
    int *refCountPtr;
    int refCount;
    undefined result;
    char *localPropertyName;
    size_t localLength;
    ushort *key;

    localPropertyName = propertyName;
    localLength = strlen(propertyName);
    StringASCII::StringASCII((StringASCII *)&key, (CharSequence *)&localPropertyName);
    result = getValue(this, key, value);

		// if the string is empty, delete the object
    if ((undefined4 *)(key - 6) != &std::basic_string<unsigned_short, std::char_traits<unsigned_short>, std::allocator<unsigned_short>>::_Rep::_S_empty_rep_storage) {
        refCountPtr = (int *)(key - 2);
        refCount = *refCountPtr;
        *refCountPtr = *refCountPtr - 1;
        if (refCount < 1) {
            delete(key - 6);
        }
    }

    return result;
}
